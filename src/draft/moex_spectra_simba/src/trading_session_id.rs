#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[derive(Default)]
pub enum TradingSessionID {
    Day = 0x1_u8, 
    Morning = 0x3_u8, 
    Evening = 0x5_u8, 
    #[default]
    NullVal = 0xff_u8, 
}

impl From<u8> for TradingSessionID {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Day, 
            0x3_u8 => Self::Morning, 
            0x5_u8 => Self::Evening, 
            _ => Self::NullVal,
        }
    }
}
