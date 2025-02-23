use std::ops::Add;

#[derive(Debug)]
pub struct BigInt {
    num: Vec<u8>,
}

pub trait ToBigInt {
    fn to_bigint(self) -> Result<BigInt, &'static str>;
}

impl ToBigInt for &str {
    fn to_bigint(self) -> Result<BigInt, &'static str> {
        let mut num: Vec<u8> = Vec::new();

        for c in self.chars() {
            match c.to_digit(10) {
                Some(digit) => num.push(digit as u8),
                None => return Err("It's not a digit."),
            }
        }

        Ok( BigInt { num } )
    }
}

impl Add for BigInt {
    type Output = BigInt;
    
    fn add(self, rhs: Self) -> Self::Output {
        let self_len = self.num.len();
        let rhs_len = rhs.num.len();
        let big_len = if self_len >= rhs_len { self_len } else { rhs_len };
        
        let mut result: Vec<u8> = vec![0; big_len + 1];

        for i in 0..big_len {
            // here
        }

        BigInt { num: result }
    }
}

impl BigInt {
    pub fn new() -> Self {
        Self { num: Vec::new() }
    }

    pub fn from(s: &str) -> Self {
        match s.to_bigint() {
            Ok(num) => num,
            Err(e) => panic!("{e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bi1 = "632".to_bigint().unwrap();
        let bi2 = BigInt::from("13");
        let bi3 = bi1 + bi2;

        println!("3: {:?}", bi3);
    }
}