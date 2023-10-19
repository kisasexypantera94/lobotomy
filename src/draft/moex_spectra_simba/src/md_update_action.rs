#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[derive(Default)]
pub enum MDUpdateAction {
    New = 0x0_u8, 
    Change = 0x1_u8, 
    Delete = 0x2_u8, 
    #[default]
    NullVal = 0xff_u8, 
}

impl From<u8> for MDUpdateAction {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::New, 
            0x1_u8 => Self::Change, 
            0x2_u8 => Self::Delete, 
            _ => Self::NullVal,
        }
    }
}
