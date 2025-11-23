//! Arbitrary-precision decimal number implementation.
//!
//! This module provides the [`Decimal`] type for representing and performing
//! arithmetic operations on decimal numbers with arbitrary precision.

use crate::bigint::BigInt;
use crate::DecimalError;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::str::FromStr;

/// An arbitrary-precision decimal number.
///
/// `Decimal` represents a decimal number as a signed coefficient and scale:
/// value = coefficient × 10^(-scale)
///
/// # Examples
///
/// ```
/// use decimal_rs::decimal::Decimal;
/// use std::str::FromStr;
///
/// let a = Decimal::from_str("123.45").unwrap();
/// let b = Decimal::from_str("67.89").unwrap();
/// let c = a + b;
/// assert_eq!(format!("{}", c), "191.34");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decimal {
    /// Signed coefficient (the significand)
    coefficient: BigInt,
    /// Number of decimal places (scale)
    /// The actual value is coefficient × 10^(-scale)
    scale: u32,
}

impl Decimal {
    /// Creates a new `Decimal` from a coefficient and scale.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::decimal::Decimal;
    /// use decimal_rs::bigint::BigInt;
    ///
    /// // 123.45 = 12345 × 10^(-2)
    /// let dec = Decimal::new(BigInt::from(12345_i64), 2);
    /// assert_eq!(format!("{}", dec), "123.45");
    /// ```
    pub fn new(coefficient: BigInt, scale: u32) -> Self {
        Self { coefficient, scale }
    }

    /// Creates a zero `Decimal`.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::decimal::Decimal;
    ///
    /// let zero = Decimal::zero();
    /// assert_eq!(format!("{}", zero), "0");
    /// ```
    pub fn zero() -> Self {
        Self {
            coefficient: BigInt::from(0_i64),
            scale: 0,
        }
    }

    /// Returns true if the decimal is zero.
    pub fn is_zero(&self) -> bool {
        use crate::bigint::Zero;
        self.coefficient.is_zero()
    }

    /// Normalizes the decimal by removing trailing zeros from the coefficient.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::decimal::Decimal;
    /// use decimal_rs::bigint::BigInt;
    ///
    /// // 1.2300 → 1.23
    /// let mut dec = Decimal::new(BigInt::from(12300_i64), 4);
    /// dec.normalize();
    /// assert_eq!(format!("{}", dec), "1.23");
    /// ```
    pub fn normalize(&mut self) {
        use crate::bigint::Zero;

        if self.coefficient.is_zero() {
            self.scale = 0;
            return;
        }

        // Remove trailing zeros by dividing by 10
        while self.scale > 0 {
            let ten = BigInt::from(10_i64);
            let remainder = self.coefficient.clone() % ten.clone();

            if !remainder.is_zero() {
                break;
            }

            self.coefficient = self.coefficient.clone() / ten;
            self.scale -= 1;
        }
    }

    /// Aligns two decimals to have the same scale.
    /// Returns (aligned_a, aligned_b, common_scale)
    fn align_scales(a: &Decimal, b: &Decimal) -> (BigInt, BigInt, u32) {
        let max_scale = a.scale.max(b.scale);

        let a_coef = if a.scale < max_scale {
            let power = max_scale - a.scale;
            a.coefficient.clone() * Self::power_of_10(power)
        } else {
            a.coefficient.clone()
        };

        let b_coef = if b.scale < max_scale {
            let power = max_scale - b.scale;
            b.coefficient.clone() * Self::power_of_10(power)
        } else {
            b.coefficient.clone()
        };

        (a_coef, b_coef, max_scale)
    }

    /// Returns 10^n as a BigInt
    fn power_of_10(n: u32) -> BigInt {
        if n == 0 {
            return BigInt::from(1_i64);
        }

        let mut result = BigInt::from(1_i64);
        for _ in 0..n {
            result = result * BigInt::from(10_i64);
        }
        result
    }
}

impl FromStr for Decimal {
    type Err = DecimalError;

    /// Parses a string into a `Decimal`.
    ///
    /// Supports formats: "123", "123.45", "-123.45", "+123.45", ".5", "-.5"
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::decimal::Decimal;
    /// use std::str::FromStr;
    ///
    /// let a = Decimal::from_str("123.45").unwrap();
    /// let b = Decimal::from_str("-0.001").unwrap();
    /// let c = Decimal::from_str("100").unwrap();
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(DecimalError::EmptyInput);
        }

        let s = s.trim();

        // Handle sign
        let (sign_char, s) = if s.starts_with('-') {
            ('-', &s[1..])
        } else if s.starts_with('+') {
            ('+', &s[1..])
        } else {
            ('+', s)
        };

        if s.is_empty() {
            return Err(DecimalError::EmptyInput);
        }

        // Split by decimal point
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() > 2 {
            return Err(DecimalError::InvalidFormat(
                "Multiple decimal points".to_string(),
            ));
        }

        let integer_part = if parts[0].is_empty() { "0" } else { parts[0] };
        let fractional_part = if parts.len() == 2 { parts[1] } else { "" };

        // Validate characters
        for c in integer_part.chars() {
            if !c.is_ascii_digit() {
                return Err(DecimalError::InvalidDigit(c));
            }
        }

        for c in fractional_part.chars() {
            if !c.is_ascii_digit() {
                return Err(DecimalError::InvalidDigit(c));
            }
        }

        // Build coefficient string (without decimal point)
        let coef_str = if sign_char == '-' {
            format!("-{}{}", integer_part, fractional_part)
        } else {
            format!("{}{}", integer_part, fractional_part)
        };

        // Parse coefficient
        let coefficient = BigInt::from_str(&coef_str)?;

        // Scale is the number of fractional digits
        let scale = fractional_part.len() as u32;

        let mut result = Self { coefficient, scale };
        result.normalize();
        Ok(result)
    }
}

impl fmt::Display for Decimal {
    /// Formats the `Decimal` as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::decimal::Decimal;
    /// use std::str::FromStr;
    ///
    /// let dec = Decimal::from_str("123.45").unwrap();
    /// assert_eq!(format!("{}", dec), "123.45");
    ///
    /// let neg = Decimal::from_str("-0.001").unwrap();
    /// assert_eq!(format!("{}", neg), "-0.001");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::bigint::Zero;

        if self.coefficient.is_zero() {
            return write!(f, "0");
        }

        let coef_str = format!("{}", self.coefficient);
        let is_negative = coef_str.starts_with('-');
        let digits = if is_negative {
            &coef_str[1..]
        } else {
            &coef_str
        };

        if self.scale == 0 {
            // No decimal point
            write!(f, "{}", coef_str)
        } else if self.scale >= digits.len() as u32 {
            // Need leading zeros: 0.00...digits
            let leading_zeros = self.scale as usize - digits.len();
            write!(
                f,
                "{}0.{}{}",
                if is_negative { "-" } else { "" },
                "0".repeat(leading_zeros),
                digits
            )
        } else {
            // Decimal point within digits
            let split_pos = digits.len() - self.scale as usize;
            write!(
                f,
                "{}{}.{}",
                if is_negative { "-" } else { "" },
                &digits[..split_pos],
                &digits[split_pos..]
            )
        }
    }
}

impl Add for Decimal {
    type Output = Decimal;

    /// Adds two decimals.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::decimal::Decimal;
    /// use std::str::FromStr;
    ///
    /// let a = Decimal::from_str("123.45").unwrap();
    /// let b = Decimal::from_str("67.89").unwrap();
    /// let c = a + b;
    /// assert_eq!(format!("{}", c), "191.34");
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        let (a_coef, b_coef, scale) = Decimal::align_scales(&self, &rhs);
        let result_coef = a_coef + b_coef;

        let mut result = Decimal {
            coefficient: result_coef,
            scale,
        };
        result.normalize();
        result
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    /// Subtracts two decimals.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::decimal::Decimal;
    /// use std::str::FromStr;
    ///
    /// let a = Decimal::from_str("100.5").unwrap();
    /// let b = Decimal::from_str("30.25").unwrap();
    /// let c = a - b;
    /// assert_eq!(format!("{}", c), "70.25");
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        let (a_coef, b_coef, scale) = Decimal::align_scales(&self, &rhs);
        let result_coef = a_coef - b_coef;

        let mut result = Decimal {
            coefficient: result_coef,
            scale,
        };
        result.normalize();
        result
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    /// Multiplies two decimals.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::decimal::Decimal;
    /// use std::str::FromStr;
    ///
    /// let a = Decimal::from_str("12.5").unwrap();
    /// let b = Decimal::from_str("4.2").unwrap();
    /// let c = a * b;
    /// assert_eq!(format!("{}", c), "52.5");
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        let result_coef = self.coefficient * rhs.coefficient;
        let result_scale = self.scale + rhs.scale;

        let mut result = Decimal {
            coefficient: result_coef,
            scale: result_scale,
        };
        result.normalize();
        result
    }
}

impl Div for Decimal {
    type Output = Decimal;

    /// Divides two decimals with a default precision of 18 decimal places.
    ///
    /// # Panics
    ///
    /// Panics if dividing by zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::decimal::Decimal;
    /// use std::str::FromStr;
    ///
    /// let a = Decimal::from_str("10").unwrap();
    /// let b = Decimal::from_str("4").unwrap();
    /// let c = a / b;
    /// assert_eq!(format!("{}", c), "2.5");
    /// ```
    fn div(self, rhs: Self) -> Self::Output {
        use crate::bigint::Zero;

        if rhs.coefficient.is_zero() {
            panic!("Division by zero");
        }

        // For division, we need to add precision
        // a/b = (a × 10^precision) / b, then scale by precision
        const PRECISION: u32 = 18;

        let numerator = self.coefficient * Decimal::power_of_10(PRECISION);
        let result_coef = numerator / rhs.coefficient;
        let result_scale = self.scale + PRECISION - rhs.scale;

        let mut result = Decimal {
            coefficient: result_coef,
            scale: result_scale,
        };
        result.normalize();
        result
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    /// Compares two decimals.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::decimal::Decimal;
    /// use std::str::FromStr;
    ///
    /// let a = Decimal::from_str("123.45").unwrap();
    /// let b = Decimal::from_str("67.89").unwrap();
    /// assert!(a > b);
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        // Align scales and compare coefficients
        let (a_coef, b_coef, _) = Decimal::align_scales(self, other);
        a_coef.cmp(&b_coef)
    }
}

impl From<i64> for Decimal {
    /// Creates a `Decimal` from an `i64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::decimal::Decimal;
    ///
    /// let dec = Decimal::from(12345_i64);
    /// assert_eq!(format!("{}", dec), "12345");
    ///
    /// let neg = Decimal::from(-42_i64);
    /// assert_eq!(format!("{}", neg), "-42");
    /// ```
    fn from(n: i64) -> Self {
        Decimal {
            coefficient: BigInt::from(n),
            scale: 0,
        }
    }
}

impl From<i32> for Decimal {
    fn from(n: i32) -> Self {
        Decimal::from(n as i64)
    }
}

impl From<u64> for Decimal {
    fn from(n: u64) -> Self {
        Decimal {
            coefficient: BigInt::from(n),
            scale: 0,
        }
    }
}

impl Default for Decimal {
    fn default() -> Self {
        Decimal::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_new() {
        let dec = Decimal::new(BigInt::from(12345_i64), 2);
        assert_eq!(format!("{}", dec), "123.45");
    }

    #[test]
    fn test_decimal_zero() {
        let zero = Decimal::zero();
        assert!(zero.is_zero());
        assert_eq!(format!("{}", zero), "0");
    }

    #[test]
    fn test_parse_simple() {
        let dec = Decimal::from_str("123.45").unwrap();
        assert_eq!(format!("{}", dec), "123.45");
    }

    #[test]
    fn test_parse_negative() {
        let dec = Decimal::from_str("-123.45").unwrap();
        assert_eq!(format!("{}", dec), "-123.45");
    }

    #[test]
    fn test_parse_integer() {
        let dec = Decimal::from_str("100").unwrap();
        assert_eq!(format!("{}", dec), "100");
    }

    #[test]
    fn test_parse_leading_dot() {
        let dec = Decimal::from_str(".5").unwrap();
        assert_eq!(format!("{}", dec), "0.5");
    }

    #[test]
    fn test_parse_small_decimal() {
        let dec = Decimal::from_str("0.001").unwrap();
        assert_eq!(format!("{}", dec), "0.001");
    }

    #[test]
    fn test_normalize() {
        let mut dec = Decimal::new(BigInt::from(12300_i64), 4);
        dec.normalize();
        assert_eq!(format!("{}", dec), "1.23");
    }

    #[test]
    fn test_normalize_zero() {
        let mut dec = Decimal::new(BigInt::from(0_i64), 5);
        dec.normalize();
        assert_eq!(format!("{}", dec), "0");
    }

    // Arithmetic operation tests
    #[test]
    fn test_add_simple() {
        let a = Decimal::from_str("123.45").unwrap();
        let b = Decimal::from_str("67.89").unwrap();
        let c = a + b;
        assert_eq!(format!("{}", c), "191.34");
    }

    #[test]
    fn test_add_different_scales() {
        let a = Decimal::from_str("10.5").unwrap();
        let b = Decimal::from_str("2.25").unwrap();
        let c = a + b;
        assert_eq!(format!("{}", c), "12.75");
    }

    #[test]
    fn test_add_negative() {
        let a = Decimal::from_str("10.5").unwrap();
        let b = Decimal::from_str("-3.2").unwrap();
        let c = a + b;
        assert_eq!(format!("{}", c), "7.3");
    }

    #[test]
    fn test_sub_simple() {
        let a = Decimal::from_str("100.5").unwrap();
        let b = Decimal::from_str("30.25").unwrap();
        let c = a - b;
        assert_eq!(format!("{}", c), "70.25");
    }

    #[test]
    fn test_sub_negative_result() {
        let a = Decimal::from_str("10").unwrap();
        let b = Decimal::from_str("15").unwrap();
        let c = a - b;
        assert_eq!(format!("{}", c), "-5");
    }

    #[test]
    fn test_mul_simple() {
        let a = Decimal::from_str("12.5").unwrap();
        let b = Decimal::from_str("4.2").unwrap();
        let c = a * b;
        assert_eq!(format!("{}", c), "52.5");
    }

    #[test]
    fn test_mul_by_ten() {
        let a = Decimal::from_str("1.23").unwrap();
        let b = Decimal::from_str("10").unwrap();
        let c = a * b;
        assert_eq!(format!("{}", c), "12.3");
    }

    #[test]
    fn test_mul_negative() {
        let a = Decimal::from_str("-2.5").unwrap();
        let b = Decimal::from_str("4").unwrap();
        let c = a * b;
        assert_eq!(format!("{}", c), "-10");
    }

    #[test]
    fn test_div_simple() {
        let a = Decimal::from_str("10").unwrap();
        let b = Decimal::from_str("4").unwrap();
        let c = a / b;
        assert_eq!(format!("{}", c), "2.5");
    }

    #[test]
    fn test_div_with_decimals() {
        let a = Decimal::from_str("7.5").unwrap();
        let b = Decimal::from_str("2.5").unwrap();
        let c = a / b;
        assert_eq!(format!("{}", c), "3");
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_div_by_zero() {
        let a = Decimal::from_str("10").unwrap();
        let b = Decimal::zero();
        let _ = a / b;
    }
}
