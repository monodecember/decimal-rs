// 주의! 이 모듈은 decimal-rs 에서 작동하도록 만들어졌으며, 일반적인 사용에 적합하지 않음.

pub struct BigU {
    num: Vec<u32>,
}

impl BigU {
    fn from(s: String) -> Self {
        let mut num: Vec<u32> = Vec::new();

        match s.parse::<u32>() {
            Ok(ok_num) => num.push(ok_num),
            Err(e) => match e.kind() {
                    std::num::IntErrorKind::PosOverflow => { num = to_bigu(s); },
                    _ => panic!("Not a valid integer."),
            },
        }

        Self { num }
    } // or 모든 정수를 u8 으로 표현하기. 3213 = [3, 2, 1, 3] 자리수 계산이 편한 대신 느릴 것.
}

fn to_bigu(s: String) -> Vec<u32> {
    Vec::new()
}
