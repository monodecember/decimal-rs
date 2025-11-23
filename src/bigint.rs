//! Arbitrary-precision integer implementation.
//!
//! This module provides the [`BigInt`] type for representing and performing
//! arithmetic operations on integers of arbitrary size.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub, Mul};
use std::str::FromStr;
use crate::DecimalError;

/// An arbitrary-precision integer.
///
/// `BigInt` stores integers as a vector of digits (0-9), allowing it to
/// represent numbers larger than the native integer types.
///
/// # Examples
///
/// ```
/// use decimal_rs::bigint::BigInt;
///
/// let a = BigInt::from("12345");
/// let b = BigInt::from("67890");
/// let c = a + b;
/// assert_eq!(c, BigInt::from("80235"));
/// ```
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct BigInt {
    /// Vector of digits (0-9) representing the number
    num: Vec<u8>,
}

impl FromStr for BigInt {
    type Err = DecimalError;

    /// Parses a string into a `BigInt`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The string is empty ([`DecimalError::EmptyInput`])
    /// - The string contains non-digit characters ([`DecimalError::InvalidDigit`])
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    /// use std::str::FromStr;
    ///
    /// let num = BigInt::from_str("12345").unwrap();
    /// assert_eq!(num, BigInt::from("12345"));
    ///
    /// let result = BigInt::from_str("abc");
    /// assert!(result.is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(DecimalError::EmptyInput);
        }

        let mut num: Vec<u8> = Vec::new();

        for c in s.chars() {
            match c.to_digit(10) {
                Some(digit) => num.push(digit as u8),
                None => return Err(DecimalError::InvalidDigit(c)),
            }
        }

        Ok(Self { num })
    }
}

impl From<&str> for BigInt {
    /// Converts a string slice to a `BigInt`.
    ///
    /// # Panics
    ///
    /// Panics if the string cannot be parsed into a valid `BigInt`.
    /// For a non-panicking version, use [`FromStr::from_str`] or the `parse()` method.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from("12345");
    /// ```
    fn from(s: &str) -> Self {
        s.parse().expect("Failed to parse BigInt from string")
    }
}

impl Add for BigInt {
    type Output = BigInt;

    /// Adds two `BigInt` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from("999");
    /// let b = BigInt::from("1");
    /// let c = a + b;
    /// assert_eq!(c, BigInt::from("1000"));
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        let mut self_num = self.num;
        let mut rhs_num = rhs.num;

        self_num.reverse();
        rhs_num.reverse();

        let self_len = self_num.len();
        let rhs_len = rhs_num.len();
        let big_len = if self_len >= rhs_len { self_len } else { rhs_len };

        let mut result: Vec<u8> = vec![0; big_len];
        let mut carry = 0;

        for i in 0..big_len {
            let s_num = if i < self_len { self_num[i] } else { 0 };
            let r_num = if i < rhs_len { rhs_num[i] } else { 0 };

            let mut sum = s_num + r_num + carry;

            if sum >= 10 {
                carry = 1;
                sum -= 10;
            } else {
                carry = 0;
            }

            result[i] = sum;
        }

        if carry == 1 {
            result.push(1);
        }

        result.reverse();

        BigInt { num: result }
    }
}

impl Sub for BigInt {
    type Output = BigInt;

    /// Subtracts one `BigInt` from another.
    ///
    /// # Panics
    ///
    /// Panics if `self` is less than `rhs`, as negative numbers are not supported.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from("100");
    /// let b = BigInt::from("42");
    /// let c = a - b;
    /// assert_eq!(c, BigInt::from("58"));
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        if self < rhs {
            panic!("Subtraction would result in negative number (not supported)");
        }

        let mut self_num = self.num;
        let mut rhs_num = rhs.num;

        self_num.reverse();
        rhs_num.reverse();

        let self_len = self_num.len();
        let rhs_len = rhs_num.len();

        let mut result: Vec<u8> = vec![0; self_len];
        let mut borrow = 0;

        for i in 0..self_len {
            let s_num = self_num[i];
            let r_num = if i < rhs_len { rhs_num[i] } else { 0 };

            let mut diff = s_num as i16 - r_num as i16 - borrow;

            if diff < 0 {
                borrow = 1;
                diff += 10;
            } else {
                borrow = 0;
            }

            result[i] = diff as u8;
        }

        result.reverse();

        // Remove leading zeros
        while result.len() > 1 && result[0] == 0 {
            result.remove(0);
        }

        BigInt { num: result }
    }
}

impl Mul for BigInt {
    type Output = BigInt;

    /// Multiplies two `BigInt` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from("123");
    /// let b = BigInt::from("456");
    /// let c = a * b;
    /// assert_eq!(c, BigInt::from("56088"));
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        // Handle zero cases
        if self.num.is_empty() || rhs.num.is_empty() {
            return BigInt::new();
        }

        let self_num = &self.num;
        let rhs_num = &rhs.num;

        let self_len = self_num.len();
        let rhs_len = rhs_num.len();

        // Result can have at most self_len + rhs_len digits
        let mut result = vec![0u16; self_len + rhs_len];

        // Multiply each digit of self with each digit of rhs
        for (i, &s_digit) in self_num.iter().rev().enumerate() {
            for (j, &r_digit) in rhs_num.iter().rev().enumerate() {
                result[i + j] += (s_digit as u16) * (r_digit as u16);
            }
        }

        // Handle carries
        let mut carry = 0u16;
        for digit in result.iter_mut() {
            *digit += carry;
            carry = *digit / 10;
            *digit %= 10;
        }

        // Convert u16 to u8 and reverse
        let mut final_result: Vec<u8> = result.iter().map(|&d| d as u8).collect();
        final_result.reverse();

        // Remove leading zeros
        while final_result.len() > 1 && final_result[0] == 0 {
            final_result.remove(0);
        }

        BigInt { num: final_result }
    }
}

impl BigInt {
    /// Creates a new empty `BigInt`.
    ///
    /// The resulting `BigInt` represents zero (no digits).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::new();
    /// ```
    pub fn new() -> Self {
        Self { num: Vec::new() }
    }
}

impl fmt::Display for BigInt {
    /// Formats the `BigInt` as a decimal string.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from("12345");
    /// assert_eq!(format!("{}", num), "12345");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.num.is_empty() {
            return write!(f, "0");
        }

        for digit in &self.num {
            write!(f, "{}", digit)?;
        }
        Ok(())
    }
}

impl PartialOrd for BigInt {
    /// Compares two `BigInt` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from("100");
    /// let b = BigInt::from("99");
    /// assert!(a > b);
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    /// Compares two `BigInt` values.
    ///
    /// Returns `Ordering::Greater` if `self` is greater than `other`,
    /// `Ordering::Less` if `self` is less than `other`, and
    /// `Ordering::Equal` if they are equal.
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare lengths first
        if self.num.len() != other.num.len() {
            return self.num.len().cmp(&other.num.len());
        }

        // If lengths are equal, compare digit by digit from most significant
        for (s, o) in self.num.iter().zip(other.num.iter()) {
            match s.cmp(o) {
                Ordering::Equal => continue,
                other => return other,
            }
        }

        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_1() {
        let bi1 = BigInt::from("632");
        let bi2 = BigInt::from("13");
        let bi3 = bi1 + bi2;

        assert_eq!(bi3, BigInt::from("645"));
    }

    #[test]
    fn test_add_2() {
        let bi1 = BigInt::from("1");
        let bi2 = BigInt::from("99");
        let bi3 = bi1 + bi2;

        assert_eq!(bi3, BigInt::from("100"));
    }

    #[test]
    fn test_from_str_valid() {
        let result = "12345".parse::<BigInt>();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BigInt::from("12345"));
    }

    #[test]
    fn test_from_str_invalid_digit() {
        let result = "123a45".parse::<BigInt>();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DecimalError::InvalidDigit('a'));
    }

    #[test]
    fn test_from_str_empty() {
        let result = "".parse::<BigInt>();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DecimalError::EmptyInput);
    }

    #[test]
    fn test_from_str_special_chars() {
        let result = "123-456".parse::<BigInt>();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DecimalError::InvalidDigit('-'));
    }

    #[test]
    #[should_panic(expected = "Failed to parse BigInt from string")]
    fn test_from_panics_on_invalid() {
        let _bi = BigInt::from("abc");
    }

    // Edge case tests
    #[test]
    fn test_add_zero() {
        let bi1 = BigInt::from("0");
        let bi2 = BigInt::from("0");
        let bi3 = bi1 + bi2;
        assert_eq!(bi3, BigInt::from("0"));
    }

    #[test]
    fn test_add_zero_to_number() {
        let bi1 = BigInt::from("12345");
        let bi2 = BigInt::from("0");
        let bi3 = bi1 + bi2;
        assert_eq!(bi3, BigInt::from("12345"));
    }

    #[test]
    fn test_add_number_to_zero() {
        let bi1 = BigInt::from("0");
        let bi2 = BigInt::from("67890");
        let bi3 = bi1 + bi2;
        assert_eq!(bi3, BigInt::from("67890"));
    }

    #[test]
    fn test_add_with_multiple_carries() {
        let bi1 = BigInt::from("999");
        let bi2 = BigInt::from("1");
        let bi3 = bi1 + bi2;
        assert_eq!(bi3, BigInt::from("1000"));
    }

    #[test]
    fn test_add_carry_chain() {
        let bi1 = BigInt::from("9999999");
        let bi2 = BigInt::from("1");
        let bi3 = bi1 + bi2;
        assert_eq!(bi3, BigInt::from("10000000"));
    }

    #[test]
    fn test_add_single_digits() {
        let bi1 = BigInt::from("5");
        let bi2 = BigInt::from("3");
        let bi3 = bi1 + bi2;
        assert_eq!(bi3, BigInt::from("8"));
    }

    #[test]
    fn test_add_single_digit_with_carry() {
        let bi1 = BigInt::from("7");
        let bi2 = BigInt::from("8");
        let bi3 = bi1 + bi2;
        assert_eq!(bi3, BigInt::from("15"));
    }

    #[test]
    fn test_add_large_numbers() {
        let bi1 = BigInt::from("123456789012345678901234567890");
        let bi2 = BigInt::from("987654321098765432109876543210");
        let bi3 = bi1 + bi2;
        assert_eq!(bi3, BigInt::from("1111111110111111111011111111100"));
    }

    #[test]
    fn test_add_very_different_lengths() {
        let bi1 = BigInt::from("1");
        let bi2 = BigInt::from("123456789012345");
        let bi3 = bi1 + bi2;
        assert_eq!(bi3, BigInt::from("123456789012346"));
    }

    #[test]
    fn test_add_same_numbers() {
        let bi1 = BigInt::from("555");
        let bi2 = BigInt::from("555");
        let bi3 = bi1 + bi2;
        assert_eq!(bi3, BigInt::from("1110"));
    }

    #[test]
    fn test_new_creates_empty() {
        let bi = BigInt::new();
        assert_eq!(bi.num.len(), 0);
    }

    #[test]
    fn test_add_with_new() {
        let bi1 = BigInt::new();
        let bi2 = BigInt::from("42");
        let bi3 = bi1 + bi2;
        // Empty BigInt (no digits) + 42 should give 42
        assert_eq!(bi3, BigInt::from("42"));
    }

    // Display tests
    #[test]
    fn test_display() {
        let bi = BigInt::from("12345");
        assert_eq!(format!("{}", bi), "12345");
    }

    #[test]
    fn test_display_zero() {
        let bi = BigInt::new();
        assert_eq!(format!("{}", bi), "0");
    }

    #[test]
    fn test_display_large() {
        let bi = BigInt::from("999999999999999999");
        assert_eq!(format!("{}", bi), "999999999999999999");
    }

    // Comparison tests
    #[test]
    fn test_cmp_equal() {
        let a = BigInt::from("123");
        let b = BigInt::from("123");
        assert_eq!(a, b);
    }

    #[test]
    fn test_cmp_greater() {
        let a = BigInt::from("200");
        let b = BigInt::from("100");
        assert!(a > b);
    }

    #[test]
    fn test_cmp_less() {
        let a = BigInt::from("50");
        let b = BigInt::from("100");
        assert!(a < b);
    }

    #[test]
    fn test_cmp_different_lengths() {
        let a = BigInt::from("1000");
        let b = BigInt::from("999");
        assert!(a > b);
    }

    // Subtraction tests
    #[test]
    fn test_sub_basic() {
        let a = BigInt::from("100");
        let b = BigInt::from("42");
        let c = a - b;
        assert_eq!(c, BigInt::from("58"));
    }

    #[test]
    fn test_sub_same() {
        let a = BigInt::from("555");
        let b = BigInt::from("555");
        let c = a - b;
        assert_eq!(c, BigInt::from("0"));
    }

    #[test]
    fn test_sub_with_borrow() {
        let a = BigInt::from("1000");
        let b = BigInt::from("1");
        let c = a - b;
        assert_eq!(c, BigInt::from("999"));
    }

    #[test]
    fn test_sub_large() {
        let a = BigInt::from("123456789");
        let b = BigInt::from("987654");
        let c = a - b;
        assert_eq!(c, BigInt::from("122469135"));
    }

    #[test]
    #[should_panic(expected = "Subtraction would result in negative number")]
    fn test_sub_negative_panic() {
        let a = BigInt::from("10");
        let b = BigInt::from("20");
        let _c = a - b;
    }

    // Multiplication tests
    #[test]
    fn test_mul_basic() {
        let a = BigInt::from("123");
        let b = BigInt::from("456");
        let c = a * b;
        assert_eq!(c, BigInt::from("56088"));
    }

    #[test]
    fn test_mul_by_zero() {
        let a = BigInt::from("999");
        let b = BigInt::new();
        let c = a * b;
        assert_eq!(c, BigInt::new());
    }

    #[test]
    fn test_mul_by_one() {
        let a = BigInt::from("12345");
        let b = BigInt::from("1");
        let c = a * b;
        assert_eq!(c, BigInt::from("12345"));
    }

    #[test]
    fn test_mul_single_digits() {
        let a = BigInt::from("7");
        let b = BigInt::from("8");
        let c = a * b;
        assert_eq!(c, BigInt::from("56"));
    }

    #[test]
    fn test_mul_large() {
        let a = BigInt::from("9999");
        let b = BigInt::from("9999");
        let c = a * b;
        assert_eq!(c, BigInt::from("99980001"));
    }

    // Clone test
    #[test]
    fn test_clone() {
        let a = BigInt::from("12345");
        let b = a.clone();
        assert_eq!(a, b);
    }
}