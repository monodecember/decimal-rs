use crate::bigint::BigInt;

const DEFAULT_PRECISION: usize = 28;

#[derive(Debug)]
pub struct Decimal {
    sign: bool,
    num: BigInt,
    exponent: usize,
    pub prec: usize,
}

impl Decimal {
    pub fn new(prec: usize) -> Self {
        Self { sign: false, num: BigInt::new(), exponent: 0, prec}
    }

    pub fn debug_display(&self) {
        println!("num: {}{:?}, prec: {}", &self.sign, &self.num, &self.prec);
    }
}
