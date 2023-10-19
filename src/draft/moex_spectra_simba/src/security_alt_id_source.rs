#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[derive(Default)]
pub enum SecurityAltIDSource {
    ISIN = 52_u8, 
    ExchangeSymbol = 56_u8, 
    #[default]
    NullVal = 0_u8, 
}

impl From<u8> for SecurityAltIDSource {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            52_u8 => Self::ISIN, 
            56_u8 => Self::ExchangeSymbol, 
            _ => Self::NullVal,
        }
    }
}
