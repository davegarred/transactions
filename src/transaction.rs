use crate::amount::Amount;
use crate::{ClientId, TransactionId};
use csv::StringRecord;
use serde::Deserialize;
use crate::error::TransactionParseError;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub transaction_details: TransactionDetails,
    pub client: ClientId,
    pub transaction_id: TransactionId,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CsvDeserializedTransaction {
    #[serde(rename = "type")]
    transaction_type: String,
    client: ClientId,
    #[serde(rename = "tx")]
    transaction_id: TransactionId,
    amount: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionDetails {
    Deposit(Amount),
    Withdrawal(Amount),
    Dispute,
    Resolve,
    Chargeback,
}

impl TryFrom<(&str, Option<f64>)> for TransactionDetails {
    type Error = TransactionParseError;

    fn try_from((transaction_type, amount): (&str, Option<f64>)) -> Result<Self, Self::Error> {
        match (transaction_type.to_ascii_lowercase().as_str(), amount) {
            ("deposit", Some(amount)) => Ok(Self::Deposit(amount.into())),
            ("withdrawal", Some(amount)) => Ok(Self::Withdrawal(amount.into())),
            ("dispute", None) => Ok(Self::Dispute),
            ("resolve", None) => Ok(Self::Resolve),
            ("chargeback", None) => Ok(Self::Chargeback),
            _ => Err(format!(
                "Unknown transaction, type: {}, amount: {:?}",
                transaction_type, amount
            ).into()),
        }
    }
}

// We don't want to return a TransactionError for an error state here because additional
// information (line number) will be needed to identify where the problem lies.
impl TryFrom<StringRecord> for Transaction {
    type Error = TransactionParseError;

    fn try_from(value: StringRecord) -> Result<Self, TransactionParseError> {
        let headers = StringRecord::from(vec!["type", "client", "tx", "amount"]);
        let deser: CsvDeserializedTransaction = value.deserialize(Some(&headers))?;
        let transaction_details = (deser.transaction_type.as_str(), deser.amount).try_into()?;
        Ok(Self {
            transaction_details,
            client: deser.client,
            transaction_id: deser.transaction_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::error::TransactionParseError;
    use super::*;
    #[test]
    fn try_from_csv_type() {
        assert_eq!(
            TransactionDetails::try_from(("deposit", Some(1.5))).unwrap(),
            TransactionDetails::Deposit(1.5.into())
        );
        assert_eq!(
            TransactionDetails::try_from(("withdrawal", Some(1.5))).unwrap(),
            TransactionDetails::Withdrawal(1.5.into())
        );
        assert!(TransactionDetails::try_from(("something_else", None)).is_err());
    }

    #[test]
    fn try_from_csv() {
        assert_eq!(
            process_line(vec!["deposit", "1", "1", "1.0"]).unwrap(),
            Transaction {
                transaction_details: TransactionDetails::Deposit(Amount::new(10000)),
                client: 1,
                transaction_id: 1,
            }
        );
        assert_eq!(
            process_line(vec!["withdrawal", "2", "2", "12345.54321"]).unwrap(),
            Transaction {
                transaction_details: TransactionDetails::Withdrawal(12345.5432.into()),
                client: 2,
                transaction_id: 2,
            }
        );
    }

    fn process_line(values: Vec<&str>) -> Result<Transaction, TransactionParseError> {
        let record = StringRecord::from(values);
        Transaction::try_from(record)
    }
}
