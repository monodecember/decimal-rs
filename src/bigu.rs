pub struct BigU {
    v: Vec<u32>,
    sign: bool
}

pub trait ToBigU {
    fn to_biguint(&self) -> Option<BigU>;
}
