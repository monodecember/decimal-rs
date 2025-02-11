#[derive(Debug, PartialEq)]
pub struct BigInt {
    num: Vec<u8>,
}

trait ToBigInt {
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

    pub fn print_bigint(&self) {
        println!("{:?}", &self.num);
    }
}

// // 소유권 문제를 이해하고 최적화 하자
// impl Add for BigInt {
//     type Output = BigInt;

//     fn add(self, rhs: Self) -> Self::Output {
//         let mut result: BigInt = BigInt::new();

//         // 더 긴 길이의 벡터를 찾는다.
//         let mut left_len = (self.num).len();
//         let mut right_len = (rhs.num).len();
//         let big_len = if left_len > right_len { left_len } else { right_len };

//         // 불확실한 loop 대신 for-loop 를 사용한다.
//         // 기존에 만들었던 변수 `left_len`, `right_len` 을 인덱스로 사용한다.
//         let mut next: u8 = 0;
//         for i in 0..big_len {
//             let x: u8;

//             // -1 은 꼭 필요하다. 왜 그런지 생각해보라.
//             if left_len <= 0 && right_len > 0 {
//                 x = rhs.num[right_len - 1];
//                 right_len -= 1;
//             } else if right_len <= 0 && left_len > 0 {
//                 x = self.num[left_len - 1];
//                 left_len -= 1;
//             } else {
//                 x = self.num[left_len - 1] + rhs.num[right_len - 1];
//                 left_len -= 1;
//                 right_len -= 1;
//             }

//             result.num.insert(0, x + next);
//             next = 0;

//             // 왜 이 코드가 insert 구문 보다 밑에 있을지 생각해보라.
//             if x >= 10 {
//                 result.num[i] = x % 10;
//                 next = x / 10;
//             }
//         }

//         result
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", BigInt::from("13513734232643897236598726147894613198413218549321654894621659845036"));
    }
}