#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[derive(Default)]
pub enum NegativePrices {
    NotEligible = 0x0_u8, 
    Eligible = 0x1_u8, 
    #[default]
    NullVal = 0xff_u8, 
}

impl From<u8> for NegativePrices {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::NotEligible, 
            0x1_u8 => Self::Eligible, 
            _ => Self::NullVal,
        }
    }
}
