#[derive(Clone, Debug, PartialEq)]
pub enum TransactionError {
    FileUnreadable(String),
    FileTransaction(usize, String),
}

impl From<std::io::Error> for TransactionError {
    fn from(value: std::io::Error) -> Self {
        Self::FileUnreadable(value.to_string())
    }
}
