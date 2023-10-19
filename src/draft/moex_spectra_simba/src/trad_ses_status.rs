#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[derive(Default)]
pub enum TradSesStatus {
    Halted = 0x1_u8, 
    Open = 0x2_u8, 
    Closed = 0x3_u8, 
    PreOpen = 0x4_u8, 
    #[default]
    NullVal = 0xff_u8, 
}

impl From<u8> for TradSesStatus {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::Halted, 
            0x2_u8 => Self::Open, 
            0x3_u8 => Self::Closed, 
            0x4_u8 => Self::PreOpen, 
            _ => Self::NullVal,
        }
    }
}
