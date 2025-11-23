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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_new() {
        let dec = Decimal::new(10);
        assert_eq!(dec.prec, 10);
    }

    #[test]
    fn test_decimal_default_values() {
        let dec = Decimal::new(28);
        assert_eq!(dec.sign, false);
        assert_eq!(dec.exponent, 0);
        assert_eq!(dec.prec, 28);
    }

    #[test]
    fn test_decimal_new_with_zero_precision() {
        let dec = Decimal::new(0);
        assert_eq!(dec.prec, 0);
        assert_eq!(dec.sign, false);
    }

    #[test]
    fn test_decimal_new_with_large_precision() {
        let dec = Decimal::new(100);
        assert_eq!(dec.prec, 100);
    }

    #[test]
    fn test_decimal_uses_default_precision_constant() {
        // Verify the constant exists and has expected value
        assert_eq!(DEFAULT_PRECISION, 28);
    }
}
