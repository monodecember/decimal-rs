//! Arbitrary-precision decimal number implementation.
//!
//! This module provides the [`Decimal`] type for representing and performing
//! arithmetic operations on decimal numbers with configurable precision.

use crate::bigint::BigInt;

/// Default precision for decimal numbers (number of significant digits)
const DEFAULT_PRECISION: usize = 28;

/// An arbitrary-precision decimal number.
///
/// `Decimal` represents a decimal number with configurable precision,
/// suitable for financial calculations and other applications requiring
/// exact decimal arithmetic.
///
/// # Examples
///
/// ```
/// use decimal_rs::decimal::Decimal;
///
/// let dec = Decimal::new(10);
/// ```
#[derive(Debug, Clone)]
pub struct Decimal {
    /// Sign of the decimal (false = positive, true = negative)
    sign: bool,
    /// The underlying integer value
    num: BigInt,
    /// The exponent (decimal point position)
    exponent: usize,
    /// Precision (number of significant digits)
    pub prec: usize,
}

impl Decimal {
    /// Creates a new `Decimal` with the specified precision.
    ///
    /// The decimal is initialized to zero with the given precision.
    ///
    /// # Arguments
    ///
    /// * `prec` - The number of significant digits to maintain
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::decimal::Decimal;
    ///
    /// let dec = Decimal::new(10);
    /// ```
    pub fn new(prec: usize) -> Self {
        Self { sign: false, num: BigInt::new(), exponent: 0, prec}
    }

    /// Prints debug information about the decimal.
    ///
    /// This is a temporary debugging method that prints the internal
    /// representation of the decimal to stdout.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use decimal_rs::decimal::Decimal;
    ///
    /// let dec = Decimal::new(10);
    /// dec.debug_display();
    /// ```
    pub fn debug_display(&self) {
        println!("num: {}{:?}, prec: {}", &self.sign, &self.num, &self.prec);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_new() {
        let dec = Decimal::new(10);
        assert_eq!(dec.prec, 10);
    }

    #[test]
    fn test_decimal_default_values() {
        let dec = Decimal::new(28);
        assert_eq!(dec.sign, false);
        assert_eq!(dec.exponent, 0);
        assert_eq!(dec.prec, 28);
    }

    #[test]
    fn test_decimal_new_with_zero_precision() {
        let dec = Decimal::new(0);
        assert_eq!(dec.prec, 0);
        assert_eq!(dec.sign, false);
    }

    #[test]
    fn test_decimal_new_with_large_precision() {
        let dec = Decimal::new(100);
        assert_eq!(dec.prec, 100);
    }

    #[test]
    fn test_decimal_uses_default_precision_constant() {
        // Verify the constant exists and has expected value
        assert_eq!(DEFAULT_PRECISION, 28);
    }
}
