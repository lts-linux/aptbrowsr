//! Error and Result types.
use serde::{Deserialize, Serialize};
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Error {
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StorageError: {}", self.message)
    }
}

impl Error {
    pub fn from_error(
        error: &dyn std::error::Error,
        message: &str,
    ) -> Error {
        let message = format!("{}: {}", message, error);
        Error {
            message: message,
        }
    }

    pub fn new(
        message: &str,
    ) -> Error {
        Error {
            message: message.to_string(),
        }
    }
}
