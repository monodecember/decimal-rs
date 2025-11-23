//! # decimal-rs
//!
//! A Rust library for arbitrary-precision decimal arithmetic.
//!
//! This library provides two main types:
//! - [`BigInt`](bigint::BigInt): Arbitrary-precision integers
//! - [`Decimal`](decimal::Decimal): Arbitrary-precision decimal numbers
//!
//! ## Examples
//!
//! ```
//! use decimal_rs::bigint::BigInt;
//!
//! let a = BigInt::from("123456789");
//! let b = BigInt::from("987654321");
//! let c = a + b;
//! ```
//!
//! ## Error Handling
//!
//! The library uses [`DecimalError`] for error handling, which is returned as
//! `Result<T, DecimalError>` from fallible operations like parsing.

pub mod bigint;
pub mod decimal;

use std::fmt;

/// Errors that can occur when working with decimal numbers
///
/// This enum represents all possible errors that can occur during
/// decimal number operations, particularly during parsing and conversion.
#[derive(Debug, Clone, PartialEq)]
pub enum DecimalError {
    /// Invalid character found while parsing a number
    ///
    /// Contains the invalid character that was encountered.
    InvalidDigit(char),

    /// Input string is empty
    ///
    /// Returned when attempting to parse an empty string into a number.
    EmptyInput,

    /// Invalid format
    ///
    /// Contains a description of the format error.
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
