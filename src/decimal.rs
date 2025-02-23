use std::ops::Add;

use crate::bigint::BigInt;

const DEFAULT_PRECISION: usize = 28;

#[derive(Debug)]
pub struct Decimal {
    sign: bool,
    num: BigInt,
    exponent: usize,
    // 정확도 문제부터 수정하자. 그냥 + 연산으로만 구현하면 28로 지정되면 그 정확도로 ㄱㄱ
    pub prec: usize, // 정확도가 있어야하는 이유는 자원의 한계 때문이다.
}

impl Decimal {
    pub fn new(prec: usize) -> Self {
        // 여기서는 `0` 이 만들어지는 것이 아니다
        Self { sign: false, num: BigInt::new(), exponent: 0, prec}
    }

    // pub fn from(s: &str, prec: usize) -> Self {
    //     let v: Vec<String> = to_decimal(s);

    //     Self {
    //         sign: v[0].parse::<bool>().expect("Not a valid sign."),
    //         num: BigInt::from(&v[1]),
    //         exponent: v[2].parse::<usize>().expect("Not a valid exponent."),
    //         prec
    //     }
    // }

    pub fn debug_display(&self) {
        // BigInt 에서 `.to_string()` 을 구현하고, 다시 작성하자.
        println!("num: {}{:?}, prec: {}", &self.sign, &self.num, &self.prec);
    }
}

// impl Add for Decimal {
//     type Output = Decimal;

//     fn add(self, rhs: Self) -> Self::Output {
//         let big_prec = if self.prec > rhs.prec {self.prec} else {rhs.prec};
//         let mut result = Decimal::new(big_prec);

//         result
//     }
// }

// fn is_decimal(s: String) -> bool {
//     let mut result: bool = true;
//     for x in s.chars() {
//         match x.is_digit(10) {
//             true => continue,
//             false => match x {
//                 '.' => continue,
//                 _ => result = false
//             }
//         }
//     }

//     result
// }

fn to_decimal(num: &str) -> Vec<String> {
    let mut result_vec: Vec<String> = Vec::new();
    let mut point_pos: usize = 0;
    let mut s1: String = String::new();
    
    for (i, x) in (num).chars().enumerate() {
        // Sign
        if i == 0 {
            match x {
                '+' => result_vec.push("0".to_string()),
                '-' => result_vec.push("1".to_string()),
                _ => match x.is_digit(10) {
                    true => result_vec.push("0".to_string()),
                    false => continue
                }
            }
        }

        // Coefficient, exponent
        if x.is_digit(10) { s1.push(x); }
        else if x == '.' { point_pos += i }
    }

    result_vec.push(s1);
    result_vec.push((num.len() - point_pos - 1).to_string()); // -1 제거. 혹은, 왜 추가한 건지 주석 추가하기.
    
    result_vec
}
