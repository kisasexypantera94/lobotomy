#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[derive(Default)]
pub enum TradSesEvent {
    TradingResumes = 0x0_u8, 
    ChangeOfTradingSession = 0x1_u8, 
    ChangeOfTradingStatus = 0x3_u8, 
    #[default]
    NullVal = 0xff_u8, 
}

impl From<u8> for TradSesEvent {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::TradingResumes, 
            0x1_u8 => Self::ChangeOfTradingSession, 
            0x3_u8 => Self::ChangeOfTradingStatus, 
            _ => Self::NullVal,
        }
    }
}
