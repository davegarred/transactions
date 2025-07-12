use crate::ClientId;
use crate::client::Client;
use crate::error::TransactionError;
use crate::transaction::Transaction;
use std::collections::HashMap;

/// Handles file processing and tracks client status.
/// Clients are kept in memory but expanding this with a persistent backing
/// would enable much more robust recovery from errors, processing of more than
/// a single file, etc., etc.
#[derive(Debug, Default)]
pub struct Bank {
    clients: HashMap<ClientId, Client>,
}

impl Bank {
    /// Handles all transaction processing based on a single batch file.
    pub fn process_file(&mut self, filename: &str) -> Result<(), TransactionError> {
        let file = std::fs::File::open(filename)?;
        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .has_headers(true)
            .from_reader(file);
        for (line, result) in rdr.records().enumerate() {
            let record = match result {
                Ok(record) => record,
                Err(msg) => return Err(TransactionError::FileTransaction(line, msg.to_string())),
            };
            let transaction: Transaction = match record.try_into() {
                Ok(transaction) => transaction,
                Err(msg) => return Err(TransactionError::FileTransaction(line, msg.value())),
            };
            self.process_transaction(transaction);
        }
        Ok(())
    }

    /// Processes an individual transaction.
    /// Errors are ignored so this method is errorless.
    pub fn process_transaction(&mut self, transaction: Transaction) {
        let client = match self.clients.get_mut(&transaction.client) {
            Some(client) => client,
            None => {
                let client = Client::new(transaction.client);
                self.clients.insert(transaction.client, client);
                self.clients
                    .get_mut(&transaction.client)
                    .expect("client was just inserted, must exist")
            }
        };
        client.process_transaction(transaction);
    }

    /// A helper function to show the current state of all clients.
    pub fn write_client_status_csv_stdout(&self) {
        println!("client, available, held, total, locked");
        for client in self.clients.values() {
            println!(
                "{:6}, {:9}, {:4}, {:5}, {:6}",
                client.id(),
                client.available(),
                client.hold(),
                client.total(),
                client.locked()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn process_valid_file_no_errors() {
        let mut bank = Bank::default();
        bank.process_file("data/test_process_valid_file_no_errors.csv")
            .unwrap();
    }

    #[test]
    fn error_on_malformed_txn() {
        let mut bank = Bank::default();
        bank.process_file("data/test_error_on_malformed_txn.csv")
            .unwrap_err();
    }
}
