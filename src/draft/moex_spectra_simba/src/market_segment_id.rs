#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[derive(Default)]
pub enum MarketSegmentID {
    Derivatives = 68_u8, 
    #[default]
    NullVal = 0_u8, 
}

impl From<u8> for MarketSegmentID {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            68_u8 => Self::Derivatives, 
            _ => Self::NullVal,
        }
    }
}
