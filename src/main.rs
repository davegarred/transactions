#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(rust_2018_idioms)]
#![doc = include_str!("../README.md")]
//!

use crate::bank::Bank;
use crate::error::TransactionError;
use std::process::exit;

mod amount;
mod bank;
mod client;
mod error;
mod transaction;

/// Uniquely identifies a client.
pub type ClientId = u16;

/// Uniquely identifies a transaction.
pub type TransactionId = u32;

/// Entrypoint expects a single argument for the filename containing the batch of transactions to be processed.
/// Errors parsing the file or in converting the data to transactions will be written to stderr and
/// the application will end with an exit code of 1.
/// Any errors during transaction processing (Insufficient Funds) will not halt processing.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let mut bank = Bank::default();
    if let Err(err) = bank.process_file(&args[1]) {
        let msg = match err {
            TransactionError::FileUnreadable(msg) => {
                format!("file not found or is unreadable: {}", msg)
            }
            TransactionError::FileTransaction(line, msg) => {
                format!("error in file at transaction line {}: {}", line, msg)
            }
        };
        eprintln!("{}", msg);
        exit(1);
    }
    bank.write_client_status_csv_stdout();
}
