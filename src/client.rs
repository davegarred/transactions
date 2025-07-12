use crate::amount::Amount;
use crate::transaction::{Transaction, TransactionDetails};
use crate::{ClientId, TransactionId};
use std::collections::{HashMap, hash_map};

/// Stores all details of a client's account status.
#[derive(Debug)]
pub struct Client {
    id: ClientId,
    transactions: HashMap<TransactionId, TransactionDetails>,
    disputed_transactions: HashMap<TransactionId, TransactionDetails>,
    locked: bool,
}

impl Client {
    pub fn new(id: ClientId) -> Self {
        Self {
            id,
            transactions: Default::default(),
            disputed_transactions: Default::default(),
            locked: false,
        }
    }

    pub fn id(&self) -> ClientId {
        self.id
    }

    /// Current available funds
    pub fn available(&self) -> f64 {
        self.sum_transactions(self.transactions.values()).into()
    }

    /// Current funds on hold
    pub fn hold(&self) -> f64 {
        self.sum_transactions(self.disputed_transactions.values())
            .into()
    }

    /// Current total funds for the account
    pub fn total(&self) -> f64 {
        let total = self.sum_transactions(self.transactions.values())
            + self.sum_transactions(self.disputed_transactions.values());
        total.into()
    }

    /// Is the account currently locked? (If so, all incoming transactions are ignored.)
    pub fn locked(&self) -> bool {
        self.locked
    }

    /// Process a single transaction. This always returns successfully as any error in the
    /// transactions is simply ignored. Actions that raise security concerns (at least in my
    /// mind) are logged to stderr.
    pub fn process_transaction(&mut self, transaction: Transaction) {
        if self.locked {
            eprintln!(
                "attempt to modify a locked client: {}, txn: {}",
                transaction.client, transaction.transaction_id
            );
            return;
        }
        match &transaction.transaction_details {
            TransactionDetails::Deposit(_) => {
                self.transactions
                    .insert(transaction.transaction_id, transaction.transaction_details);
            }
            TransactionDetails::Withdrawal(amount) => {
                let current_balance = self.available();
                let tx_amount: f64 = (*amount).into();
                if current_balance < tx_amount {
                    eprintln!(
                        "attempt to withdraw with insufficient funds for client: {}, txn: {}",
                        transaction.client, transaction.transaction_id
                    );
                    return;
                }
                self.transactions
                    .insert(transaction.transaction_id, transaction.transaction_details);
            }
            TransactionDetails::Dispute => {
                if let Some(previous_transaction) =
                    self.transactions.remove(&transaction.transaction_id)
                {
                    self.disputed_transactions
                        .insert(transaction.transaction_id, previous_transaction);
                };
            }
            TransactionDetails::Resolve => {
                if let Some(previous_transaction) = self
                    .disputed_transactions
                    .remove(&transaction.transaction_id)
                {
                    self.transactions
                        .insert(transaction.transaction_id, previous_transaction);
                };
            }
            TransactionDetails::Chargeback => {
                if self
                    .disputed_transactions
                    .remove(&transaction.transaction_id)
                    .is_some()
                {
                    self.locked = true;
                }
            }
        }
    }

    // Helper function to sum the total funds in either the transaction list or the
    // disputed transaction list.
    fn sum_transactions(
        &self,
        transactions: hash_map::Values<'_, TransactionId, TransactionDetails>,
    ) -> Amount {
        let mut sum = Amount::default();
        for transaction in transactions {
            match transaction {
                TransactionDetails::Deposit(amount) => {
                    sum = sum + *amount;
                }
                TransactionDetails::Withdrawal(amount) => {
                    sum = sum - *amount;
                }
                _ => {}
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::TransactionDetails::{
        Chargeback, Deposit, Dispute, Resolve, Withdrawal,
    };
    const TEST_CLIENT_ID: ClientId = 1337;

    #[test]
    fn deposit_withdrawal() {
        let mut client = Client::new(TEST_CLIENT_ID);
        client.process_transaction(test_transaction(Deposit(1.5.into()), 1));
        client.process_transaction(test_transaction(Deposit(4.0.into()), 2));
        client.process_transaction(test_transaction(Withdrawal(2.0.into()), 3));
        assert_eq!(client.hold(), 0.0);
        assert_eq!(client.available(), 3.5);
        assert_eq!(client.total(), 3.5);
        // ignore transaction if insufficient funds exist
        client.process_transaction(test_transaction(Withdrawal(4.0.into()), 4));
        assert_eq!(client.hold(), 0.0);
        assert_eq!(client.available(), 3.5);
        assert_eq!(client.total(), 3.5);
    }

    #[test]
    fn dispute_resolve() {
        let mut client = Client::new(TEST_CLIENT_ID);
        client.process_transaction(test_transaction(Deposit(1.5.into()), 1));
        client.process_transaction(test_transaction(Deposit(2.5.into()), 2));
        client.process_transaction(test_transaction(Dispute, 1));
        assert_eq!(client.hold(), 1.5);
        assert_eq!(client.available(), 2.5);
        assert_eq!(client.total(), 4.0);
        client.process_transaction(test_transaction(Resolve, 1));
        assert_eq!(client.hold(), 0.0);
        assert_eq!(client.available(), 4.0);
        assert_eq!(client.total(), 4.0);
    }

    #[test]
    fn chargeback() {
        let mut client = Client::new(TEST_CLIENT_ID);
        client.process_transaction(test_transaction(Deposit(1.5.into()), 1));
        client.process_transaction(test_transaction(Deposit(2.5.into()), 2));
        client.process_transaction(test_transaction(Dispute, 1));
        client.process_transaction(test_transaction(Chargeback, 1));
        assert_eq!(client.hold(), 0.0);
        assert_eq!(client.available(), 2.5);
        assert_eq!(client.total(), 2.5);
        assert!(client.locked());
        // No new transactions are processed
        client.process_transaction(test_transaction(Deposit(1.5.into()), 3));
        client.process_transaction(test_transaction(Dispute, 3));
        assert_eq!(client.hold(), 0.0);
        assert_eq!(client.available(), 2.5);
        assert_eq!(client.total(), 2.5);
    }

    #[test]
    fn chargeback_for_transaction_that_is_not_in_dispute() {
        let mut client = Client::new(TEST_CLIENT_ID);
        client.process_transaction(test_transaction(Deposit(1.5.into()), 1));
        client.process_transaction(test_transaction(Deposit(2.5.into()), 2));
        client.process_transaction(test_transaction(Chargeback, 1));
        assert_eq!(client.hold(), 0.0);
        assert_eq!(client.available(), 4.0);
        assert_eq!(client.total(), 4.0);
        assert!(!client.locked());
    }

    fn test_transaction(
        transaction_details: TransactionDetails,
        transaction_id: TransactionId,
    ) -> Transaction {
        Transaction {
            transaction_details,
            client: TEST_CLIENT_ID,
            transaction_id,
        }
    }
}
