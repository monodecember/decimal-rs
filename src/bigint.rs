//! Arbitrary-precision integer implementation.
//!
//! This module provides the [`BigInt`] type for representing and performing
//! arithmetic operations on integers of arbitrary size.

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, Shl, Shr};
use std::str::FromStr;
use crate::DecimalError;

/// Trait for types that have a zero value.
pub trait Zero {
    /// Returns the zero value for this type.
    fn zero() -> Self;

    /// Returns `true` if this value is zero.
    fn is_zero(&self) -> bool;
}

/// Trait for types that have a multiplicative identity (one).
pub trait One {
    /// Returns the multiplicative identity (one) for this type.
    fn one() -> Self;

    /// Returns `true` if this value is one.
    fn is_one(&self) -> bool;
}

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
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
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

impl From<u64> for BigInt {
    /// Converts a `u64` to a `BigInt`.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(12345_u64);
    /// assert_eq!(num, BigInt::from("12345"));
    /// ```
    fn from(mut n: u64) -> Self {
        if n == 0 {
            return Self { num: vec![0] };
        }

        let mut digits = Vec::new();
        while n > 0 {
            digits.push((n % 10) as u8);
            n /= 10;
        }
        digits.reverse();

        Self { num: digits }
    }
}

impl From<u8> for BigInt {
    /// Converts a `u8` to a `BigInt`.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(255_u8);
    /// assert_eq!(num, BigInt::from("255"));
    /// ```
    fn from(n: u8) -> Self {
        Self::from(n as u64)
    }
}

impl From<u16> for BigInt {
    /// Converts a `u16` to a `BigInt`.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(65535_u16);
    /// assert_eq!(num, BigInt::from("65535"));
    /// ```
    fn from(n: u16) -> Self {
        Self::from(n as u64)
    }
}

impl From<u32> for BigInt {
    /// Converts a `u32` to a `BigInt`.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(12345_u32);
    /// assert_eq!(num, BigInt::from("12345"));
    /// ```
    fn from(n: u32) -> Self {
        Self::from(n as u64)
    }
}

impl From<usize> for BigInt {
    /// Converts a `usize` to a `BigInt`.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(12345_usize);
    /// assert_eq!(num, BigInt::from("12345"));
    /// ```
    fn from(n: usize) -> Self {
        Self::from(n as u64)
    }
}

impl TryFrom<i64> for BigInt {
    type Error = DecimalError;

    /// Tries to convert an `i64` to a `BigInt`.
    ///
    /// Returns an error if the number is negative, as `BigInt` only supports
    /// non-negative numbers.
    ///
    /// # Errors
    ///
    /// Returns [`DecimalError::InvalidFormat`] if the number is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    /// use std::convert::TryFrom;
    ///
    /// let num = BigInt::try_from(12345_i64).unwrap();
    /// assert_eq!(num, BigInt::from("12345"));
    ///
    /// let result = BigInt::try_from(-123_i64);
    /// assert!(result.is_err());
    /// ```
    fn try_from(n: i64) -> Result<Self, Self::Error> {
        if n < 0 {
            return Err(DecimalError::InvalidFormat(
                "BigInt does not support negative numbers".to_string(),
            ));
        }
        Ok(Self::from(n as u64))
    }
}

impl TryFrom<i8> for BigInt {
    type Error = DecimalError;

    /// Tries to convert an `i8` to a `BigInt`.
    ///
    /// Returns an error if the number is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    /// use std::convert::TryFrom;
    ///
    /// let num = BigInt::try_from(127_i8).unwrap();
    /// assert_eq!(num, BigInt::from(127_u64));
    /// ```
    fn try_from(n: i8) -> Result<Self, Self::Error> {
        Self::try_from(n as i64)
    }
}

impl TryFrom<i16> for BigInt {
    type Error = DecimalError;

    /// Tries to convert an `i16` to a `BigInt`.
    ///
    /// Returns an error if the number is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    /// use std::convert::TryFrom;
    ///
    /// let num = BigInt::try_from(32767_i16).unwrap();
    /// assert_eq!(num, BigInt::from(32767_u64));
    /// ```
    fn try_from(n: i16) -> Result<Self, Self::Error> {
        Self::try_from(n as i64)
    }
}

impl TryFrom<i32> for BigInt {
    type Error = DecimalError;

    /// Tries to convert an `i32` to a `BigInt`.
    ///
    /// Returns an error if the number is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    /// use std::convert::TryFrom;
    ///
    /// let num = BigInt::try_from(12345_i32).unwrap();
    /// assert_eq!(num, BigInt::from(12345_u64));
    /// ```
    fn try_from(n: i32) -> Result<Self, Self::Error> {
        Self::try_from(n as i64)
    }
}

impl TryFrom<isize> for BigInt {
    type Error = DecimalError;

    /// Tries to convert an `isize` to a `BigInt`.
    ///
    /// Returns an error if the number is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    /// use std::convert::TryFrom;
    ///
    /// let num = BigInt::try_from(12345_isize).unwrap();
    /// assert_eq!(num, BigInt::from(12345_u64));
    /// ```
    fn try_from(n: isize) -> Result<Self, Self::Error> {
        Self::try_from(n as i64)
    }
}

impl TryFrom<BigInt> for u64 {
    type Error = DecimalError;

    /// Tries to convert a `BigInt` to a `u64`.
    ///
    /// Returns an error if the `BigInt` is too large to fit in a `u64`.
    ///
    /// # Errors
    ///
    /// Returns [`DecimalError::InvalidFormat`] if the value exceeds `u64::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    /// use std::convert::TryFrom;
    ///
    /// let big = BigInt::from(12345_u64);
    /// let n: u64 = big.try_into().unwrap();
    /// assert_eq!(n, 12345);
    ///
    /// let too_big = BigInt::from("99999999999999999999999999");
    /// let result = u64::try_from(too_big);
    /// assert!(result.is_err());
    /// ```
    fn try_from(value: BigInt) -> Result<Self, Self::Error> {
        // Handle zero case (empty or single 0)
        if value.num.is_empty() || (value.num.len() == 1 && value.num[0] == 0) {
            return Ok(0);
        }

        // Check if it fits in u64 by trying to parse
        let s = format!("{}", value);
        s.parse::<u64>().map_err(|_| {
            DecimalError::InvalidFormat(format!(
                "BigInt value {} exceeds u64::MAX ({})",
                s,
                u64::MAX
            ))
        })
    }
}

impl TryFrom<BigInt> for u32 {
    type Error = DecimalError;

    /// Tries to convert a `BigInt` to a `u32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    /// use std::convert::TryFrom;
    ///
    /// let big = BigInt::from(12345_u32);
    /// let n: u32 = big.try_into().unwrap();
    /// assert_eq!(n, 12345);
    /// ```
    fn try_from(value: BigInt) -> Result<Self, Self::Error> {
        let n = u64::try_from(value)?;
        u32::try_from(n).map_err(|_| {
            DecimalError::InvalidFormat(format!("Value {} exceeds u32::MAX", n))
        })
    }
}

impl TryFrom<BigInt> for usize {
    type Error = DecimalError;

    /// Tries to convert a `BigInt` to a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    /// use std::convert::TryFrom;
    ///
    /// let big = BigInt::from(12345_usize);
    /// let n: usize = big.try_into().unwrap();
    /// assert_eq!(n, 12345);
    /// ```
    fn try_from(value: BigInt) -> Result<Self, Self::Error> {
        let n = u64::try_from(value)?;
        usize::try_from(n).map_err(|_| {
            DecimalError::InvalidFormat(format!("Value {} exceeds usize::MAX", n))
        })
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
        let self_num = &self.num;
        let rhs_num = &rhs.num;

        let self_len = self_num.len();
        let rhs_len = rhs_num.len();
        let big_len = self_len.max(rhs_len);

        let mut result: Vec<u8> = Vec::with_capacity(big_len + 1);
        let mut carry = 0;

        // Process digits from right to left (least to most significant)
        for i in 0..big_len {
            let self_idx = self_len.saturating_sub(i + 1);
            let rhs_idx = rhs_len.saturating_sub(i + 1);

            let s_num = if i < self_len { self_num[self_idx] } else { 0 };
            let r_num = if i < rhs_len { rhs_num[rhs_idx] } else { 0 };

            let mut sum = s_num + r_num + carry;

            if sum >= 10 {
                carry = 1;
                sum -= 10;
            } else {
                carry = 0;
            }

            result.push(sum);
        }

        if carry == 1 {
            result.push(1);
        }

        // Reverse once at the end to get most-to-least significant order
        result.reverse();

        BigInt { num: result }
    }
}

impl AddAssign for BigInt {
    /// Performs `+=` operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let mut a = BigInt::from(100_u64);
    /// a += BigInt::from(50_u64);
    /// assert_eq!(a, BigInt::from(150_u64));
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
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

        let self_num = &self.num;
        let rhs_num = &rhs.num;

        let self_len = self_num.len();
        let rhs_len = rhs_num.len();

        let mut result: Vec<u8> = Vec::with_capacity(self_len);
        let mut borrow = 0;

        // Process digits from right to left (least to most significant)
        for i in 0..self_len {
            let self_idx = self_len - 1 - i;
            let rhs_idx = rhs_len.saturating_sub(i + 1);

            let s_num = self_num[self_idx];
            let r_num = if i < rhs_len { rhs_num[rhs_idx] } else { 0 };

            let mut diff = s_num as i16 - r_num as i16 - borrow;

            if diff < 0 {
                borrow = 1;
                diff += 10;
            } else {
                borrow = 0;
            }

            result.push(diff as u8);
        }

        // Reverse once at the end to get most-to-least significant order
        result.reverse();

        // Remove leading zeros - O(n) instead of O(n²)
        let leading_zeros = result.iter().take_while(|&&d| d == 0).count();
        let start = if leading_zeros >= result.len() - 1 {
            result.len() - 1
        } else {
            leading_zeros
        };

        BigInt { num: result[start..].to_vec() }
    }
}

impl SubAssign for BigInt {
    /// Performs `-=` operation.
    ///
    /// # Panics
    ///
    /// Panics if the result would be negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let mut a = BigInt::from(100_u64);
    /// a -= BigInt::from(30_u64);
    /// assert_eq!(a, BigInt::from(70_u64));
    /// ```
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
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
        // Process from right to left (least to most significant)
        for i in 0..self_len {
            let self_idx = self_len - 1 - i;
            for j in 0..rhs_len {
                let rhs_idx = rhs_len - 1 - j;
                result[i + j] += (self_num[self_idx] as u16) * (rhs_num[rhs_idx] as u16);
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

        // Remove leading zeros - O(n) instead of O(n²)
        let leading_zeros = final_result.iter().take_while(|&&d| d == 0).count();
        let start = if leading_zeros >= final_result.len() - 1 {
            final_result.len() - 1
        } else {
            leading_zeros
        };

        BigInt { num: final_result[start..].to_vec() }
    }
}

impl MulAssign for BigInt {
    /// Performs `*=` operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let mut a = BigInt::from(10_u64);
    /// a *= BigInt::from(5_u64);
    /// assert_eq!(a, BigInt::from(50_u64));
    /// ```
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl Shl<usize> for BigInt {
    type Output = BigInt;

    /// Left shift (multiply by 10^rhs).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(123_u64);
    /// let shifted = num << 2;  // Multiply by 100
    /// assert_eq!(shifted, BigInt::from(12300_u64));
    /// ```
    fn shl(self, rhs: usize) -> Self::Output {
        if rhs == 0 || self.is_zero() {
            return self;
        }

        // Append 'rhs' zeros to the end
        let mut result = self.num;
        result.reserve(rhs);
        for _ in 0..rhs {
            result.push(0);
        }

        BigInt { num: result }
    }
}

impl Shr<usize> for BigInt {
    type Output = BigInt;

    /// Right shift (divide by 10^rhs).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(12300_u64);
    /// let shifted = num >> 2;  // Divide by 100
    /// assert_eq!(shifted, BigInt::from(123_u64));
    /// ```
    fn shr(self, rhs: usize) -> Self::Output {
        if rhs == 0 {
            return self;
        }

        if rhs >= self.num.len() {
            return BigInt::zero();
        }

        // Remove 'rhs' digits from the end
        let new_len = self.num.len() - rhs;
        BigInt { num: self.num[..new_len].to_vec() }
    }
}

impl Div for BigInt {
    type Output = BigInt;

    /// Divides one `BigInt` by another.
    ///
    /// # Panics
    ///
    /// Panics if `rhs` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from(100_u64);
    /// let b = BigInt::from(7_u64);
    /// let c = a / b;
    /// assert_eq!(c, BigInt::from(14_u64));
    /// ```
    fn div(self, rhs: Self) -> Self::Output {
        // Check for division by zero
        if rhs.num.is_empty() || (rhs.num.len() == 1 && rhs.num[0] == 0) {
            panic!("Division by zero");
        }

        // Handle zero dividend
        if self.num.is_empty() || (self.num.len() == 1 && self.num[0] == 0) {
            return BigInt::from(0_u64);
        }

        // If divisor is larger, result is 0
        if self < rhs {
            return BigInt::from(0_u64);
        }

        // If equal, result is 1
        if self == rhs {
            return BigInt::from(1_u64);
        }

        // Long division algorithm
        let mut quotient = Vec::new();
        let mut remainder = BigInt::from(0_u64);

        for &digit in &self.num {
            // Shift remainder left and add next digit
            if remainder == BigInt::from(0_u64) {
                // If remainder is zero, replace it with the new digit
                remainder = if digit == 0 {
                    BigInt::from(0_u64)
                } else {
                    BigInt { num: vec![digit] }
                };
            } else {
                // Otherwise append the digit
                remainder.num.push(digit);
            }

            // Find how many times rhs fits into current remainder
            let mut count = 0_u8;
            while remainder >= rhs {
                remainder = remainder.checked_sub(&rhs).unwrap();
                count += 1;
            }

            // Add to quotient (skip leading zeros)
            if !quotient.is_empty() || count > 0 {
                quotient.push(count);
            }
        }

        // Handle case where quotient is empty (result is 0)
        if quotient.is_empty() {
            return BigInt::from(0_u64);
        }

        BigInt { num: quotient }
    }
}

impl Rem for BigInt {
    type Output = BigInt;

    /// Computes the remainder of `self` / `rhs`.
    ///
    /// # Panics
    ///
    /// Panics if `rhs` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from(100_u64);
    /// let b = BigInt::from(7_u64);
    /// let c = a % b;
    /// assert_eq!(c, BigInt::from(2_u64));
    /// ```
    fn rem(self, rhs: Self) -> Self::Output {
        // Check for division by zero
        if rhs.num.is_empty() || (rhs.num.len() == 1 && rhs.num[0] == 0) {
            panic!("Division by zero");
        }

        // Handle zero dividend
        if self.num.is_empty() || (self.num.len() == 1 && self.num[0] == 0) {
            return BigInt::from(0_u64);
        }

        // If divisor is larger, remainder is self
        if self < rhs {
            return self;
        }

        // If equal, remainder is 0
        if self == rhs {
            return BigInt::from(0_u64);
        }

        // Long division to find remainder
        let mut remainder = BigInt::from(0_u64);

        for &digit in &self.num {
            // Shift remainder left and add next digit
            if remainder == BigInt::from(0_u64) {
                // If remainder is zero, replace it with the new digit
                remainder = if digit == 0 {
                    BigInt::from(0_u64)
                } else {
                    BigInt { num: vec![digit] }
                };
            } else {
                // Otherwise append the digit
                remainder.num.push(digit);
            }

            // Subtract rhs as many times as possible
            while remainder >= rhs {
                remainder = remainder.checked_sub(&rhs).unwrap();
            }
        }

        remainder
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

    /// Checked subtraction. Returns `None` if the result would be negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from(100_u64);
    /// let b = BigInt::from(42_u64);
    /// assert_eq!(a.checked_sub(&b), Some(BigInt::from(58_u64)));
    ///
    /// let c = BigInt::from(10_u64);
    /// let d = BigInt::from(20_u64);
    /// assert_eq!(c.checked_sub(&d), None);  // Would be negative
    /// ```
    pub fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        if self < rhs {
            None
        } else {
            Some(self.clone() - rhs.clone())
        }
    }

    /// Checked addition. Returns `None` on overflow (never for BigInt).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from(100_u64);
    /// let b = BigInt::from(42_u64);
    /// assert_eq!(a.checked_add(&b), Some(BigInt::from(142_u64)));
    /// ```
    pub fn checked_add(&self, rhs: &Self) -> Option<Self> {
        // BigInt never overflows
        Some(self.clone() + rhs.clone())
    }

    /// Checked multiplication. Returns `None` on overflow (never for BigInt).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from(10_u64);
    /// let b = BigInt::from(5_u64);
    /// assert_eq!(a.checked_mul(&b), Some(BigInt::from(50_u64)));
    /// ```
    pub fn checked_mul(&self, rhs: &Self) -> Option<Self> {
        // BigInt never overflows
        Some(self.clone() * rhs.clone())
    }

    /// Checked division. Returns `None` if dividing by zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from(100_u64);
    /// let b = BigInt::from(5_u64);
    /// assert_eq!(a.checked_div(&b), Some(BigInt::from(20_u64)));
    ///
    /// let zero = BigInt::from(0_u64);
    /// assert_eq!(a.checked_div(&zero), None);
    /// ```
    pub fn checked_div(&self, rhs: &Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            Some(self.clone() / rhs.clone())
        }
    }

    /// Checked remainder. Returns `None` if dividing by zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from(100_u64);
    /// let b = BigInt::from(7_u64);
    /// assert_eq!(a.checked_rem(&b), Some(BigInt::from(2_u64)));
    ///
    /// let zero = BigInt::from(0_u64);
    /// assert_eq!(a.checked_rem(&zero), None);
    /// ```
    pub fn checked_rem(&self, rhs: &Self) -> Option<Self> {
        if rhs.is_zero() {
            None
        } else {
            Some(self.clone() % rhs.clone())
        }
    }

    /// Saturating addition (same as normal add for BigInt).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from(100_u64);
    /// let b = BigInt::from(42_u64);
    /// assert_eq!(a.saturating_add(&b), BigInt::from(142_u64));
    /// ```
    pub fn saturating_add(&self, rhs: &Self) -> Self {
        // BigInt never saturates
        self.clone() + rhs.clone()
    }

    /// Saturating subtraction. Returns zero if result would be negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::{BigInt, Zero};
    ///
    /// let a = BigInt::from(100_u64);
    /// let b = BigInt::from(42_u64);
    /// assert_eq!(a.saturating_sub(&b), BigInt::from(58_u64));
    ///
    /// let c = BigInt::from(10_u64);
    /// let d = BigInt::from(20_u64);
    /// assert!(c.saturating_sub(&d).is_zero());
    /// ```
    pub fn saturating_sub(&self, rhs: &Self) -> Self {
        self.checked_sub(rhs).unwrap_or_else(BigInt::zero)
    }

    /// Saturating multiplication (same as normal mul for BigInt).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from(10_u64);
    /// let b = BigInt::from(5_u64);
    /// assert_eq!(a.saturating_mul(&b), BigInt::from(50_u64));
    /// ```
    pub fn saturating_mul(&self, rhs: &Self) -> Self {
        // BigInt never saturates
        self.clone() * rhs.clone()
    }

    /// Returns `true` if this number is even.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let even = BigInt::from(42_u64);
    /// let odd = BigInt::from(43_u64);
    /// assert!(even.is_even());
    /// assert!(!odd.is_even());
    /// ```
    pub fn is_even(&self) -> bool {
        if self.num.is_empty() {
            return true; // Zero is even
        }
        self.num[self.num.len() - 1] % 2 == 0
    }

    /// Returns `true` if this number is odd.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let even = BigInt::from(42_u64);
    /// let odd = BigInt::from(43_u64);
    /// assert!(!even.is_odd());
    /// assert!(odd.is_odd());
    /// ```
    pub fn is_odd(&self) -> bool {
        !self.is_even()
    }

    /// Raises `self` to the power of `exp`.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let base = BigInt::from(2_u64);
    /// let result = base.pow(10);
    /// assert_eq!(result, BigInt::from(1024_u64));
    /// ```
    pub fn pow(&self, mut exp: u32) -> Self {
        if exp == 0 {
            return BigInt::from(1_u64);
        }
        if exp == 1 {
            return self.clone();
        }

        // Exponentiation by squaring
        let mut base = self.clone();
        let mut result = BigInt::from(1_u64);

        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base.clone();
            }
            base = base.clone() * base;
            exp /= 2;
        }

        result
    }

    /// Computes the greatest common divisor (GCD) using Euclid's algorithm.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from(48_u64);
    /// let b = BigInt::from(18_u64);
    /// let gcd = a.gcd(&b);
    /// assert_eq!(gcd, BigInt::from(6_u64));
    /// ```
    pub fn gcd(&self, other: &Self) -> Self {
        let mut a = self.clone();
        let mut b = other.clone();

        while !b.is_zero() {
            let remainder = a % b.clone();
            a = b;
            b = remainder;
        }

        a
    }

    /// Computes the least common multiple (LCM).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from(12_u64);
    /// let b = BigInt::from(18_u64);
    /// let lcm = a.lcm(&b);
    /// assert_eq!(lcm, BigInt::from(36_u64));
    /// ```
    pub fn lcm(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return BigInt::zero();
        }
        (self.clone() * other.clone()) / self.gcd(other)
    }

    /// Formats the number with thousand separators.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from("1234567890");
    /// assert_eq!(num.to_string_with_separator(','), "1,234,567,890");
    /// ```
    pub fn to_string_with_separator(&self, separator: char) -> String {
        let s = format!("{}", self);
        let mut result = String::new();
        let chars: Vec<char> = s.chars().collect();

        for (i, &c) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i) % 3 == 0 {
                result.push(separator);
            }
            result.push(c);
        }

        result
    }

    /// Converts the number to the specified radix (base).
    ///
    /// # Arguments
    ///
    /// * `radix` - The base to convert to (2-36)
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not in the range 2-36.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(255_u64);
    /// assert_eq!(num.to_string_radix(16), "ff");
    /// assert_eq!(num.to_string_radix(2), "11111111");
    /// ```
    pub fn to_string_radix(&self, radix: u32) -> String {
        assert!(radix >= 2 && radix <= 36, "Radix must be in range 2-36");

        if self.is_zero() {
            return "0".to_string();
        }

        const DIGITS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";

        let mut result = Vec::new();
        let mut num = self.clone();
        let radix_bigint = BigInt::from(radix as u64);

        while !num.is_zero() {
            let remainder = num.clone() % radix_bigint.clone();
            // Convert BigInt remainder to u8
            let digit = if remainder.num.is_empty() {
                0
            } else if remainder.num.len() == 1 {
                remainder.num[0]
            } else {
                // Multi-digit remainder, convert to number
                let mut val = 0u8;
                for &d in &remainder.num {
                    val = val * 10 + d;
                }
                val
            };
            result.push(DIGITS[digit as usize] as char);
            num = num / radix_bigint.clone();
        }

        result.reverse();
        result.iter().collect()
    }

    /// Returns a scientific notation string (e.g., "1.23e5").
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(123000_u64);
    /// assert_eq!(num.to_scientific_notation(), "1.23e5");
    ///
    /// let small = BigInt::from(42_u64);
    /// assert_eq!(small.to_scientific_notation(), "4.2e1");
    /// ```
    pub fn to_scientific_notation(&self) -> String {
        if self.is_zero() {
            return "0e0".to_string();
        }

        let s = format!("{}", self);
        let len = s.len();

        if len == 1 {
            return format!("{}e0", s);
        }

        // Format as: first_digit.remaining_digits e (len-1)
        let mut result = String::new();
        result.push(s.chars().next().unwrap());

        // Add decimal point and remaining significant digits
        let remaining: String = s.chars().skip(1).take_while(|&c| c != '0').collect();
        if !remaining.is_empty() {
            result.push('.');
            result.push_str(&remaining);
        }

        result.push('e');
        result.push_str(&(len - 1).to_string());

        result
    }

    /// Returns the number of digits in the number.
    ///
    /// Note: Zero has length 0 (empty internal representation).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(12345_u64);
    /// assert_eq!(num.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.num.len()
    }

    /// Returns `true` if the internal representation is empty.
    ///
    /// Note: `BigInt::new()` creates an empty representation, but
    /// `BigInt::from(0_u64)` creates `vec![0]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let zero = BigInt::new();
    /// assert!(zero.is_empty());
    ///
    /// let num = BigInt::from(42_u64);
    /// assert!(!num.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.num.is_empty()
    }

    /// Returns the individual digits of the number.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(12345_u64);
    /// assert_eq!(num.digits(), &[1, 2, 3, 4, 5]);
    /// ```
    pub fn digits(&self) -> &[u8] {
        &self.num
    }

    /// Returns the maximum of two numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from(42_u64);
    /// let b = BigInt::from(100_u64);
    /// assert_eq!(BigInt::max(a, b), BigInt::from(100_u64));
    /// ```
    pub fn max(self, other: Self) -> Self {
        if self >= other {
            self
        } else {
            other
        }
    }

    /// Returns the minimum of two numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let a = BigInt::from(42_u64);
    /// let b = BigInt::from(100_u64);
    /// assert_eq!(BigInt::min(a, b), BigInt::from(42_u64));
    /// ```
    pub fn min(self, other: Self) -> Self {
        if self <= other {
            self
        } else {
            other
        }
    }

    /// Clamps the value within a range.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(150_u64);
    /// let min = BigInt::from(0_u64);
    /// let max = BigInt::from(100_u64);
    /// assert_eq!(num.clamp(min, max), BigInt::from(100_u64));
    /// ```
    pub fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min <= max, "min must be less than or equal to max");
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    /// Returns the absolute value (no-op for unsigned BigInt).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(42_u64);
    /// assert_eq!(num.abs(), BigInt::from(42_u64));
    /// ```
    pub fn abs(self) -> Self {
        self // BigInt is always non-negative
    }

    /// Computes the factorial of this number.
    ///
    /// # Panics
    ///
    /// Panics if the number is too large (> 100000 for safety).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(5_u64);
    /// assert_eq!(num.factorial(), BigInt::from(120_u64));
    ///
    /// let zero = BigInt::from(0_u64);
    /// assert_eq!(zero.factorial(), BigInt::from(1_u64));
    /// ```
    pub fn factorial(&self) -> Self {
        if self.is_zero() || self.is_one() {
            return BigInt::one();
        }

        // Safety check to prevent extremely long computations
        if self > &BigInt::from(100000_u64) {
            panic!("Factorial input too large (max 100000)");
        }

        let mut result = BigInt::one();
        let mut i = BigInt::from(2_u64);
        let mut current = self.clone();

        while i <= current {
            result = result * i.clone();
            i = i + BigInt::one();
        }

        result
    }

    /// Converts the number to a byte vector (big-endian BCD).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(12345_u64);
    /// assert_eq!(num.to_bytes(), vec![1, 2, 3, 4, 5]);
    /// ```
    pub fn to_bytes(&self) -> Vec<u8> {
        self.num.clone()
    }

    /// Creates a BigInt from a byte vector (big-endian BCD).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let bytes = vec![1, 2, 3, 4, 5];
    /// let num = BigInt::from_bytes(&bytes);
    /// assert_eq!(num, BigInt::from(12345_u64));
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Self {
        BigInt { num: bytes.to_vec() }
    }
}

impl Default for BigInt {
    /// Creates a default `BigInt` (zero).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::default();
    /// assert_eq!(num, BigInt::new());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl Zero for BigInt {
    /// Returns the zero value.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::{BigInt, Zero};
    ///
    /// let zero = BigInt::zero();
    /// assert!(zero.is_zero());
    /// assert_eq!(format!("{}", zero), "0");
    /// ```
    fn zero() -> Self {
        BigInt::new()
    }

    /// Returns `true` if this value is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::{BigInt, Zero};
    ///
    /// let zero = BigInt::from(0_u64);
    /// let non_zero = BigInt::from(42_u64);
    /// assert!(zero.is_zero());
    /// assert!(!non_zero.is_zero());
    /// ```
    fn is_zero(&self) -> bool {
        self.num.is_empty() || (self.num.len() == 1 && self.num[0] == 0)
    }
}

impl One for BigInt {
    /// Returns the multiplicative identity (one).
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::{BigInt, One};
    ///
    /// let one = BigInt::one();
    /// assert!(one.is_one());
    /// assert_eq!(one, BigInt::from(1_u64));
    /// ```
    fn one() -> Self {
        BigInt::from(1_u64)
    }

    /// Returns `true` if this value is one.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::{BigInt, One};
    ///
    /// let one = BigInt::from(1_u64);
    /// let not_one = BigInt::from(42_u64);
    /// assert!(one.is_one());
    /// assert!(!not_one.is_one());
    /// ```
    fn is_one(&self) -> bool {
        self.num.len() == 1 && self.num[0] == 1
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

impl fmt::Binary for BigInt {
    /// Formats the `BigInt` as a binary string.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(255_u64);
    /// assert_eq!(format!("{:b}", num), "11111111");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_radix(2))
    }
}

impl fmt::Octal for BigInt {
    /// Formats the `BigInt` as an octal string.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(64_u64);
    /// assert_eq!(format!("{:o}", num), "100");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_radix(8))
    }
}

impl fmt::LowerHex for BigInt {
    /// Formats the `BigInt` as a lowercase hexadecimal string.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(255_u64);
    /// assert_eq!(format!("{:x}", num), "ff");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_radix(16))
    }
}

impl fmt::UpperHex for BigInt {
    /// Formats the `BigInt` as an uppercase hexadecimal string.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(255_u64);
    /// assert_eq!(format!("{:X}", num), "FF");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_radix(16).to_uppercase())
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

// Reference operations for better ergonomics
impl<'a, 'b> Add<&'b BigInt> for &'a BigInt {
    type Output = BigInt;

    fn add(self, rhs: &'b BigInt) -> BigInt {
        self.clone() + rhs.clone()
    }
}

impl<'a> Add<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn add(self, rhs: &'a BigInt) -> BigInt {
        self + rhs.clone()
    }
}

impl<'a> Add<BigInt> for &'a BigInt {
    type Output = BigInt;

    fn add(self, rhs: BigInt) -> BigInt {
        self.clone() + rhs
    }
}

impl<'a, 'b> Sub<&'b BigInt> for &'a BigInt {
    type Output = BigInt;

    fn sub(self, rhs: &'b BigInt) -> BigInt {
        self.clone() - rhs.clone()
    }
}

impl<'a> Sub<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: &'a BigInt) -> BigInt {
        self - rhs.clone()
    }
}

impl<'a> Sub<BigInt> for &'a BigInt {
    type Output = BigInt;

    fn sub(self, rhs: BigInt) -> BigInt {
        self.clone() - rhs
    }
}

impl<'a, 'b> Mul<&'b BigInt> for &'a BigInt {
    type Output = BigInt;

    fn mul(self, rhs: &'b BigInt) -> BigInt {
        self.clone() * rhs.clone()
    }
}

impl<'a> Mul<&'a BigInt> for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: &'a BigInt) -> BigInt {
        self * rhs.clone()
    }
}

impl<'a> Mul<BigInt> for &'a BigInt {
    type Output = BigInt;

    fn mul(self, rhs: BigInt) -> BigInt {
        self.clone() * rhs
    }
}

// PartialEq with primitive types
impl PartialEq<u64> for BigInt {
    fn eq(&self, other: &u64) -> bool {
        *self == BigInt::from(*other)
    }
}

impl PartialEq<BigInt> for u64 {
    fn eq(&self, other: &BigInt) -> bool {
        BigInt::from(*self) == *other
    }
}

impl PartialEq<u32> for BigInt {
    fn eq(&self, other: &u32) -> bool {
        *self == BigInt::from(*other)
    }
}

impl PartialEq<BigInt> for u32 {
    fn eq(&self, other: &BigInt) -> bool {
        BigInt::from(*self) == *other
    }
}

// Index trait for digit access
impl std::ops::Index<usize> for BigInt {
    type Output = u8;

    /// Access individual digits by index (0 = most significant).
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use decimal_rs::bigint::BigInt;
    ///
    /// let num = BigInt::from(12345_u64);
    /// assert_eq!(num[0], 1);
    /// assert_eq!(num[4], 5);
    /// ```
    fn index(&self, index: usize) -> &u8 {
        &self.num[index]
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

    // Default test
    #[test]
    fn test_default() {
        let a = BigInt::default();
        let b = BigInt::new();
        assert_eq!(a, b);
    }

    // Hash test
    #[test]
    fn test_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let key = BigInt::from(12345_u64);
        map.insert(key.clone(), "value");
        assert_eq!(map.get(&key), Some(&"value"));
    }

    // From<u64> tests
    #[test]
    fn test_from_u64() {
        assert_eq!(BigInt::from(0_u64), BigInt::from("0"));
        assert_eq!(BigInt::from(123_u64), BigInt::from("123"));
        assert_eq!(BigInt::from(u64::MAX), BigInt::from("18446744073709551615"));
    }

    #[test]
    fn test_from_u32() {
        assert_eq!(BigInt::from(12345_u32), BigInt::from("12345"));
    }

    #[test]
    fn test_from_usize() {
        assert_eq!(BigInt::from(12345_usize), BigInt::from("12345"));
    }

    // TryFrom<i64> tests
    #[test]
    fn test_try_from_i64_positive() {
        use std::convert::TryFrom;
        let result = BigInt::try_from(12345_i64);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BigInt::from(12345_u64));
    }

    #[test]
    fn test_try_from_i64_zero() {
        use std::convert::TryFrom;
        let result = BigInt::try_from(0_i64);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BigInt::from(0_u64));
    }

    #[test]
    fn test_try_from_i64_negative() {
        use std::convert::TryFrom;
        let result = BigInt::try_from(-123_i64);
        assert!(result.is_err());
    }

    #[test]
    fn test_try_from_i32() {
        use std::convert::TryFrom;
        assert_eq!(BigInt::try_from(12345_i32).unwrap(), BigInt::from(12345_u64));
        assert!(BigInt::try_from(-1_i32).is_err());
    }

    // TryFrom<BigInt> for u64 tests
    #[test]
    fn test_try_into_u64() {
        use std::convert::TryInto;
        let big = BigInt::from(12345_u64);
        let n: Result<u64, _> = big.try_into();
        assert!(n.is_ok());
        assert_eq!(n.unwrap(), 12345);
    }

    #[test]
    fn test_try_into_u64_zero() {
        use std::convert::TryInto;
        let big = BigInt::new();
        let n: Result<u64, _> = big.try_into();
        assert_eq!(n.unwrap(), 0);
    }

    #[test]
    fn test_try_into_u64_max() {
        use std::convert::TryInto;
        let big = BigInt::from(u64::MAX);
        let n: Result<u64, _> = big.try_into();
        assert_eq!(n.unwrap(), u64::MAX);
    }

    #[test]
    fn test_try_into_u64_overflow() {
        use std::convert::TryInto;
        let big = BigInt::from("99999999999999999999999999");
        let n: Result<u64, _> = big.try_into();
        assert!(n.is_err());
    }

    #[test]
    fn test_try_into_u32() {
        use std::convert::TryInto;
        let big = BigInt::from(12345_u32);
        let n: Result<u32, _> = big.try_into();
        assert_eq!(n.unwrap(), 12345);
    }

    // Round-trip conversion test
    #[test]
    fn test_roundtrip_u64() {
        use std::convert::TryInto;
        let original = 9876543210_u64;
        let big = BigInt::from(original);
        let converted: u64 = big.try_into().unwrap();
        assert_eq!(original, converted);
    }

    // checked_sub tests
    #[test]
    fn test_checked_sub_valid() {
        let a = BigInt::from("100");
        let b = BigInt::from("42");
        let result = a.checked_sub(&b);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), BigInt::from("58"));
    }

    #[test]
    fn test_checked_sub_equal() {
        let a = BigInt::from("555");
        let b = BigInt::from("555");
        let result = a.checked_sub(&b);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), BigInt::from("0"));
    }

    #[test]
    fn test_checked_sub_would_be_negative() {
        let a = BigInt::from("10");
        let b = BigInt::from("20");
        let result = a.checked_sub(&b);
        assert!(result.is_none());
    }

    #[test]
    fn test_checked_sub_with_borrow() {
        let a = BigInt::from("1000");
        let b = BigInt::from("1");
        let result = a.checked_sub(&b);
        assert_eq!(result.unwrap(), BigInt::from("999"));
    }

    #[test]
    fn test_checked_sub_large() {
        let a = BigInt::from("123456789");
        let b = BigInt::from("987654");
        let result = a.checked_sub(&b);
        assert_eq!(result.unwrap(), BigInt::from("122469135"));
    }

    // AddAssign tests
    #[test]
    fn test_add_assign_basic() {
        let mut a = BigInt::from("100");
        let b = BigInt::from("50");
        a += b;
        assert_eq!(a, BigInt::from("150"));
    }

    #[test]
    fn test_add_assign_with_carry() {
        let mut a = BigInt::from("999");
        let b = BigInt::from("1");
        a += b;
        assert_eq!(a, BigInt::from("1000"));
    }

    #[test]
    fn test_add_assign_zero() {
        let mut a = BigInt::from("12345");
        let b = BigInt::from("0");
        a += b;
        assert_eq!(a, BigInt::from("12345"));
    }

    // SubAssign tests
    #[test]
    fn test_sub_assign_basic() {
        let mut a = BigInt::from("100");
        let b = BigInt::from("42");
        a -= b;
        assert_eq!(a, BigInt::from("58"));
    }

    #[test]
    fn test_sub_assign_to_zero() {
        let mut a = BigInt::from("555");
        let b = BigInt::from("555");
        a -= b;
        assert_eq!(a, BigInt::from("0"));
    }

    #[test]
    fn test_sub_assign_with_borrow() {
        let mut a = BigInt::from("1000");
        let b = BigInt::from("1");
        a -= b;
        assert_eq!(a, BigInt::from("999"));
    }

    #[test]
    #[should_panic(expected = "Subtraction would result in negative number")]
    fn test_sub_assign_negative_panic() {
        let mut a = BigInt::from("10");
        let b = BigInt::from("20");
        a -= b;
    }

    // MulAssign tests
    #[test]
    fn test_mul_assign_basic() {
        let mut a = BigInt::from("12");
        let b = BigInt::from("5");
        a *= b;
        assert_eq!(a, BigInt::from("60"));
    }

    #[test]
    fn test_mul_assign_by_zero() {
        let mut a = BigInt::from("999");
        let b = BigInt::from("0");
        a *= b;
        assert_eq!(a, BigInt::from("0"));
    }

    #[test]
    fn test_mul_assign_by_one() {
        let mut a = BigInt::from("12345");
        let b = BigInt::from("1");
        a *= b;
        assert_eq!(a, BigInt::from("12345"));
    }

    #[test]
    fn test_mul_assign_large() {
        let mut a = BigInt::from("123");
        let b = BigInt::from("456");
        a *= b;
        assert_eq!(a, BigInt::from("56088"));
    }

    // Division tests
    #[test]
    fn test_div_basic() {
        let a = BigInt::from("100");
        let b = BigInt::from("5");
        let c = a / b;
        assert_eq!(c, BigInt::from("20"));
    }

    #[test]
    fn test_div_equal() {
        let a = BigInt::from("555");
        let b = BigInt::from("555");
        let c = a / b;
        assert_eq!(c, BigInt::from("1"));
    }

    #[test]
    fn test_div_remainder() {
        let a = BigInt::from("100");
        let b = BigInt::from("7");
        let c = a / b;
        assert_eq!(c, BigInt::from("14"));
    }

    #[test]
    fn test_div_exact() {
        let a = BigInt::from("144");
        let b = BigInt::from("12");
        let c = a / b;
        assert_eq!(c, BigInt::from("12"));
    }

    #[test]
    fn test_div_large() {
        let a = BigInt::from("123456789");
        let b = BigInt::from("12345");
        let c = a / b;
        assert_eq!(c, BigInt::from("10000"));
    }

    #[test]
    fn test_div_smaller_by_larger() {
        let a = BigInt::from("5");
        let b = BigInt::from("10");
        let c = a / b;
        assert_eq!(c, BigInt::from("0"));
    }

    #[test]
    fn test_div_by_one() {
        let a = BigInt::from("12345");
        let b = BigInt::from("1");
        let c = a / b;
        assert_eq!(c, BigInt::from("12345"));
    }

    #[test]
    fn test_div_zero_by_number() {
        let a = BigInt::from("0");
        let b = BigInt::from("5");
        let c = a / b;
        assert_eq!(c, BigInt::from("0"));
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_div_by_zero_panic() {
        let a = BigInt::from("100");
        let b = BigInt::from("0");
        let _c = a / b;
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_div_by_empty_panic() {
        let a = BigInt::from("100");
        let b = BigInt::new();
        let _c = a / b;
    }

    // Remainder tests
    #[test]
    fn test_rem_basic() {
        let a = BigInt::from("100");
        let b = BigInt::from("7");
        let c = a % b;
        assert_eq!(c, BigInt::from("2"));
    }

    #[test]
    fn test_rem_exact_division() {
        let a = BigInt::from("100");
        let b = BigInt::from("5");
        let c = a % b;
        assert_eq!(c, BigInt::from("0"));
    }

    #[test]
    fn test_rem_equal() {
        let a = BigInt::from("555");
        let b = BigInt::from("555");
        let c = a % b;
        assert_eq!(c, BigInt::from("0"));
    }

    #[test]
    fn test_rem_smaller_by_larger() {
        let a = BigInt::from("5");
        let b = BigInt::from("10");
        let c = a % b;
        assert_eq!(c, BigInt::from("5"));
    }

    #[test]
    fn test_rem_large() {
        let a = BigInt::from("123456789");
        let b = BigInt::from("12345");
        let c = a % b;
        assert_eq!(c, BigInt::from("6789"));
    }

    #[test]
    fn test_rem_by_one() {
        let a = BigInt::from("12345");
        let b = BigInt::from("1");
        let c = a % b;
        assert_eq!(c, BigInt::from("0"));
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_rem_by_zero_panic() {
        let a = BigInt::from("100");
        let b = BigInt::from("0");
        let _c = a % b;
    }

    // Division + Remainder verification
    #[test]
    fn test_div_rem_property() {
        // For any a, b where b != 0: a == (a / b) * b + (a % b)
        let a = BigInt::from("12345");
        let b = BigInt::from("67");
        let quotient = a.clone() / b.clone();
        let remainder = a.clone() % b.clone();
        let reconstructed = quotient * b + remainder;
        assert_eq!(a, reconstructed);
    }

    #[test]
    fn test_div_rem_property_large() {
        let a = BigInt::from("999999999");
        let b = BigInt::from("7777");
        let quotient = a.clone() / b.clone();
        let remainder = a.clone() % b.clone();
        let reconstructed = quotient * b + remainder;
        assert_eq!(a, reconstructed);
    }
}