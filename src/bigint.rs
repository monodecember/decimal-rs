use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct BigInt {
    num: Vec<u8>,
}

impl From<&str> for BigInt {
    fn from(s: &str) -> Self {
        let mut num: Vec<u8> = Vec::new();

        for c in s.chars() {
            match c.to_digit(10) {
                Some(digit) => num.push(digit as u8),
                None => panic!("Not a valid digit."),
            }
        }

        Self { num }
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
}