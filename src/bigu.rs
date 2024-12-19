// 주의! 이 모듈은 decimal-rs 에서 작동하도록 만들어졌으며, 일반적인 사용에 적합하지 않음.

use std::ops::Add;

pub struct BigU {
    num: Vec<u8>,
}

impl BigU {
    fn new() -> Self {
        Self { num: Vec::new() }
    }

    pub fn from(s: &str) -> Self {
        let mut num: Vec<u8> = Vec::new();

        for c in s.chars() {
            num.push(c.to_digit(10).expect("Num Err") as u8);
        }

        Self { num }
    }

    pub fn print_bigu(&self) {
        println!("{:?}", &self.num);
    }
}

// 소유권 문제를 이해하고 최적화 하자
impl Add for BigU {
    type Output = BigU;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result: BigU = BigU::new();

        // 더 긴 길이의 벡터를 찾는다.
        let mut left_len = (self.num).len();
        let mut right_len = (rhs.num).len();
        let big_len = if left_len > right_len { left_len } else { right_len };

        // 불확실한 loop 대신 for-loop 를 사용한다.
        // 기존에 만들었던 변수 `left_len`, `right_len` 을 인덱스로 사용한다.
        let mut next: u8 = 0;
        for i in 0..big_len {
            let mut x: u8;

            // -1 은 꼭 필요하다. 왜 그런지 생각해보라.
            if left_len <= 0 && right_len > 0 {
                x = rhs.num[right_len - 1];
                right_len -= 1;
            } else if right_len <= 0 && left_len > 0 {
                x = self.num[left_len - 1];
                left_len -= 1;
            } else {
                x = self.num[left_len - 1] + rhs.num[right_len - 1];
                left_len -= 1;
                right_len -= 1;
            }

            result.num.insert(0, x + next);
            next = 0;

            // 왜 이 코드가 insert 구문 보다 밑에 있을지 생각해보라.
            if x >= 10 {
                result.num[i] = x % 10;
                next = x / 10;
            }
        }

        result
    }
}
