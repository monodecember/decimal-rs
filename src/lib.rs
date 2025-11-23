//! # decimal-rs
//!
//! A Rust library for arbitrary-precision decimal arithmetic.
//!
//! This library provides two main types:
//! - [`BigInt`](bigint::BigInt): Arbitrary-precision integers
//! - [`Decimal`](decimal::Decimal): Arbitrary-precision decimal numbers
//!
//! It also provides numeric traits and types:
//! - [`Zero`](bigint::Zero): Types with an additive identity
//! - [`One`](bigint::One): Types with a multiplicative identity
//! - [`Sign`](bigint::Sign): Sign of a BigInt (Plus or Minus)
//!
//! ## Examples
//!
//! ```
//! use decimal_rs::bigint::BigInt;
//!
//! let a = BigInt::from("123456789");
//! let b = BigInt::from("987654321");
//! let c = a + b;
//!
//! // Signed integers
//! let d = BigInt::from(-42_i64);
//! let e = BigInt::from(10_i64);
//! let f = d + e;  // -32
//! ```
//!
//! ## Error Handling
//!
//! The library uses [`DecimalError`] for error handling, which is returned as
//! `Result<T, DecimalError>` from fallible operations like parsing.

pub mod bigint;
pub mod decimal;

// Re-export commonly used traits and types
pub use bigint::{Zero, One, Sign};

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
