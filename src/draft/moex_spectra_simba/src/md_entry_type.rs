#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[derive(Default)]
pub enum MDEntryType {
    Bid = 48_u8, 
    Offer = 49_u8, 
    EmptyBook = 74_u8, 
    #[default]
    NullVal = 0_u8, 
}

impl From<u8> for MDEntryType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::Bid, 
            49_u8 => Self::Offer, 
            74_u8 => Self::EmptyBook, 
            _ => Self::NullVal,
        }
    }
}
