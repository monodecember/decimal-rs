pub struct Decimal {
    num: Vec<String>,
    prec: usize // precision
}

impl Decimal {
    pub fn new() -> Self {
        Self { num: Vec::new(), prec: 0 }
    }

    pub fn from(s: &str, prec: usize) -> Self {
        Self { num: to_decimal(s), prec }
    }

    pub fn debug_display(&self) {
        println!("num: {:?}, prec: {}", &self.num, &self.prec);
    }
}

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
