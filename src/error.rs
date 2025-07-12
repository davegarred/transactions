use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum TransactionError {
    FileUnreadable(String),
    FileTransaction(usize, TransactionParseError),
}

impl From<std::io::Error> for TransactionError {
    fn from(value: std::io::Error) -> Self {
        Self::FileUnreadable(value.to_string())
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct TransactionParseError(String);

impl Display for TransactionParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<String> for TransactionParseError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<csv::Error> for TransactionParseError {
    fn from(err: csv::Error) -> Self {
        Self(err.to_string())
    }
}
