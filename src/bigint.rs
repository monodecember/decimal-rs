use std::ops::Add;
use std::str::FromStr;
use crate::DecimalError;

#[derive(Debug, PartialEq)]
pub struct BigInt {
    num: Vec<u8>,
}

impl FromStr for BigInt {
    type Err = DecimalError;

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
    fn from(s: &str) -> Self {
        s.parse().expect("Failed to parse BigInt from string")
    }
}

impl Add for BigInt {
    type Output = BigInt;
    
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

impl BigInt {
    pub fn new() -> Self {
        Self { num: Vec::new() }
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
}