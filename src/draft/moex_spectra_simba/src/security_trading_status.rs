#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[derive(Default)]
pub enum SecurityTradingStatus {
    TradingHalt = 0x2_u8, 
    ReadyToTrade = 0x11_u8, 
    NotAvailableForTrading = 0x12_u8, 
    NotTradedOnThisMarket = 0x13_u8, 
    UnknownOrInvalid = 0x14_u8, 
    PreOpen = 0x15_u8, 
    DiscreteAuctionOpen = 0x77_u8, 
    DiscreteAuctionClose = 0x79_u8, 
    InstrumentHalt = 0x7a_u8, 
    #[default]
    NullVal = 0xff_u8, 
}

impl From<u8> for SecurityTradingStatus {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x2_u8 => Self::TradingHalt, 
            0x11_u8 => Self::ReadyToTrade, 
            0x12_u8 => Self::NotAvailableForTrading, 
            0x13_u8 => Self::NotTradedOnThisMarket, 
            0x14_u8 => Self::UnknownOrInvalid, 
            0x15_u8 => Self::PreOpen, 
            0x77_u8 => Self::DiscreteAuctionOpen, 
            0x79_u8 => Self::DiscreteAuctionClose, 
            0x7a_u8 => Self::InstrumentHalt, 
            _ => Self::NullVal,
        }
    }
}
