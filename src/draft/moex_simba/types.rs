#![allow(dead_code)]

/// Generated by ChatGPT from C++

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct Decimal5 {
    mantissa: i64,
}

impl Decimal5 {
    pub const EXPONENT: f64 = 1e-5;
}

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct Decimal5NULL {
    pub mantissa: i64,
}

impl Decimal5NULL {
    pub const MAX_VALUE: i64 = 9223372036854775806;
    pub const NULL_VALUE: i64 = 9223372036854775807;
    pub const EXPONENT: f64 = 1e-5;
}

#[repr(u16)]
pub enum MsgFlagsSet {
    LastFragment = 0x1,
    StartOfSnapshot = 0x2,
    EndOfSnapshot = 0x4,
    IncrementalPacket = 0x8,
    PossDupFlag = 0x10,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum SecurityTradingStatus {
    TradingHalt = 2,
    ReadyToTrade = 17,
    NotAvailableForTrading = 18,
    NotTradedOnThisMarket = 19,
    UnknownOrInvalid = 20,
    PreOpen = 21,
    DiscreteAuctionOpen = 119,
    DiscreteAuctionClose = 121,
    InstrumentHalt = 122,
}

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct MarketDataPacketHeader {
    pub msg_seq_num: u32,
    pub msg_size: u16,
    pub msg_flags: u16,
    pub sending_time: u64,
}

impl MarketDataPacketHeader {
    pub fn is_last_fragment(&self) -> bool {
        (self.msg_flags & 0x1) != 0
    }

    pub fn is_incremental(&self) -> bool {
        (self.msg_flags & 0x8) != 0
    }
}

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct IncrementalPacketHeader {
    pub transact_time: u64,
    pub exchange_trading_session_id: u32,
}

impl IncrementalPacketHeader {
    pub const EXCHANGE_TRADING_SESSION_ID_NULL: u32 = 4294967295;
}

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct SBEHeader {
    pub block_length: u16,
    pub template_id: u16,
    pub schema_id: u16,
    pub version: u16,
}

impl SBEHeader {}

#[repr(u64)]
#[derive(Clone, Copy, Debug)]
pub enum MDFlagsSet {
    Day = 0x1,
    IOC = 0x2,
    NonQuote = 0x4,
    EndOfTransaction = 0x1000,
    SecondLeg = 0x4000,
    FOK = 0x80000,
    Replace = 0x100000,
    Cancel = 0x200000,
    MassCancel = 0x400000,
    Negotiated = 0x4000000,
    MultiLeg = 0x8000000,
    CrossTrade = 0x20000000,
    COD = 0x100000000,
    ActiveSide = 0x20000000000,
    PassiveSide = 0x40000000000,
    Synthetic = 0x200000000000,
    RFS = 0x400000000000,
    SyntheticPassive = 0x200000000000000,
}

#[repr(u64)]
#[derive(Clone, Copy, Debug)]
pub enum MDFlags2Set {
    Zero = 0,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum MDUpdateAction {
    New = 0,
    Change = 1,
    Delete = 2,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum MDEntryType {
    Bid = b'0',
    Offer = b'1',
    EmptyBook = b'J',
}

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct GroupSize {
    pub block_length: u16,
    pub num_in_group: u8,
}

impl GroupSize {}

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct GroupSize2 {
    pub block_length: u16,
    pub num_in_group: u16,
}

impl GroupSize2 {}

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct OrderUpdate {
    pub md_entry_id: i64,
    pub md_entry_px: Decimal5,
    pub md_entry_size: i64,
    pub md_flags: MDFlagsSet,
    pub md_flags2: MDFlags2Set,
    pub security_id: i32,
    pub rtp_seq: u32,
    pub md_update_action: MDUpdateAction,
    pub md_entry_type: MDEntryType,
}

impl OrderUpdate {
    pub const TEMPLATE_ID: u16 = 15;
}

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct OrderExecution {
    pub md_entry_id: i64,
    pub md_entry_px: Decimal5NULL,
    pub md_entry_size: i64,
    pub last_px: Decimal5,
    pub last_qty: i64,
    pub trade_id: i64,
    pub md_flags: MDFlagsSet,
    pub md_flags2: MDFlags2Set,
    pub security_id: i32,
    pub rtp_seq: u32,
    pub md_update_action: MDUpdateAction,
    pub md_entry_type: MDEntryType,
}

impl OrderExecution {
    pub const TEMPLATE_ID: u16 = 16;
}

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct BestPrices {
    pub no_md_entries: GroupSize,
}

impl BestPrices {
    pub const TEMPLATE_ID: u16 = 14;
}

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct OrderBookSnapshot {
    pub security_id: i32,
    pub last_msg_seq_num_processed: u32,
    pub rtp_seq: u32,
    pub exchange_trading_session_id: u32,
    pub no_md_entries: GroupSize,
    pub entries: [OrderBookSnapshotEntry; 0], // Placeholder for the variable-sized array
}

impl OrderBookSnapshot {
    pub const TEMPLATE_ID: u16 = 17;
}

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct OrderBookSnapshotEntry {
    pub md_entry_id: i64,
    pub transact_time: u64,
    pub md_entry_px: Decimal5NULL,
    pub md_entry_size: i64,
    pub trade_id: i64,
    pub md_flags: MDFlagsSet,
    pub md_flags2: MDFlags2Set,
    pub md_entry_type: MDEntryType,
}

impl OrderBookSnapshotEntry {}

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct SecurityMassStatus {
    pub no_related_sym: GroupSize2,
    pub security_id: i32,
    pub security_id_source: u8,
    pub security_trading_status: SecurityTradingStatus,
}

impl SecurityMassStatus {
    pub const TEMPLATE_ID: u16 = 19;
}

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
pub struct SecurityDefinition {}

#[test]
fn test_sizes() {
    assert_eq!(std::mem::size_of::<MarketDataPacketHeader>(), 16);
    assert_eq!(std::mem::size_of::<IncrementalPacketHeader>(), 12);
    assert_eq!(std::mem::size_of::<SBEHeader>(), 8);
    assert_eq!(std::mem::size_of::<OrderUpdate>(), 50);
    assert_eq!(std::mem::size_of::<OrderExecution>(), 74);
    assert_eq!(std::mem::size_of::<OrderBookSnapshotEntry>(), 57);
    assert_eq!(std::mem::size_of::<SecurityMassStatus>(), 10);
}
