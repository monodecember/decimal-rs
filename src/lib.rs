pub mod bigint;
pub mod decimal;

use std::fmt;

/// Errors that can occur when working with decimal numbers
#[derive(Debug, Clone, PartialEq)]
pub enum DecimalError {
    /// Invalid character found while parsing a number
    InvalidDigit(char),
    /// Input string is empty
    EmptyInput,
    /// Invalid format
    InvalidFormat(String),
}

impl fmt::Display for DecimalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecimalError::InvalidDigit(c) => write!(f, "Invalid digit: '{}'", c),
            DecimalError::EmptyInput => write!(f, "Cannot parse empty string"),
            DecimalError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

impl std::error::Error for DecimalError {}
