use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const SBE_BLOCK_LENGTH: u16 = 40;
pub const SBE_TEMPLATE_ID: u16 = 11;
pub const SBE_SCHEMA_ID: u16 = 19780;
pub const SBE_SCHEMA_VERSION: u16 = 3;
pub const SBE_SEMANTIC_VERSION: &str = "FIX5SP2";

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct TradingSessionStatusEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for TradingSessionStatusEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for TradingSessionStatusEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> TradingSessionStatusEncoder<'a> {
        pub fn wrap(mut self, buf: WriteBuf<'a>, offset: usize) -> Self {
            let limit = offset + SBE_BLOCK_LENGTH as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, offset: usize) -> MessageHeaderEncoder<Self> {
            let mut header = MessageHeaderEncoder::default().wrap(self, offset);
            header.block_length(SBE_BLOCK_LENGTH);
            header.template_id(SBE_TEMPLATE_ID);
            header.schema_id(SBE_SCHEMA_ID);
            header.version(SBE_SCHEMA_VERSION);
            header
        }

        /// primitive field 'TradSesOpenTime'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 8
        #[inline]
        pub fn trad_ses_open_time(&mut self, value: u64) {
            let offset = self.offset;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// primitive field 'TradSesCloseTime'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 8
        #[inline]
        pub fn trad_ses_close_time(&mut self, value: u64) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// primitive field 'TradSesIntermClearingStartTime'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 16
        /// - encodedLength: 8
        #[inline]
        pub fn trad_ses_interm_clearing_start_time(&mut self, value: u64) {
            let offset = self.offset + 16;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// primitive field 'TradSesIntermClearingEndTime'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 24
        /// - encodedLength: 8
        #[inline]
        pub fn trad_ses_interm_clearing_end_time(&mut self, value: u64) {
            let offset = self.offset + 24;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn trading_session_id(&mut self, value: TradingSessionID) {
            let offset = self.offset + 32;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'ExchangeTradingSessionID'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 33
        /// - encodedLength: 4
        #[inline]
        pub fn exchange_trading_session_id(&mut self, value: i32) {
            let offset = self.offset + 33;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn trad_ses_status(&mut self, value: TradSesStatus) {
            let offset = self.offset + 37;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        // skipping CONSTANT MarketID

        /// REQUIRED enum
        #[inline]
        pub fn market_segment_id(&mut self, value: MarketSegmentID) {
            let offset = self.offset + 38;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn trad_ses_event(&mut self, value: TradSesEvent) {
            let offset = self.offset + 39;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

    }

} // end encoder

pub mod decoder {
    use super::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct TradingSessionStatusDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for TradingSessionStatusDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for TradingSessionStatusDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> TradingSessionStatusDecoder<'a> {
        pub fn wrap(
            mut self,
            buf: ReadBuf<'a>,
            offset: usize,
            acting_block_length: u16,
            acting_version: u16,
        ) -> Self {
            let limit = offset + acting_block_length as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self.acting_block_length = acting_block_length;
            self.acting_version = acting_version;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, mut header: MessageHeaderDecoder<ReadBuf<'a>>) -> Self {
            debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
            let acting_block_length = header.block_length();
            let acting_version = header.version();

            self.wrap(
                header.parent().unwrap(),
                message_header_codec::ENCODED_LENGTH,
                acting_block_length,
                acting_version,
            )
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn trad_ses_open_time(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn trad_ses_close_time(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 8)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-1' }
        #[inline]
        pub fn trad_ses_interm_clearing_start_time(&self) -> Option<u64> {
            let value = self.get_buf().get_u64_at(self.offset + 16);
            if value == 0xffffffffffffffff_u64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-1' }
        #[inline]
        pub fn trad_ses_interm_clearing_end_time(&self) -> Option<u64> {
            let value = self.get_buf().get_u64_at(self.offset + 24);
            if value == 0xffffffffffffffff_u64 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn trading_session_id(&self) -> TradingSessionID {
            self.get_buf().get_u8_at(self.offset + 32).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-2147483648' }
        #[inline]
        pub fn exchange_trading_session_id(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 33);
            if value == -2147483648_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn trad_ses_status(&self) -> TradSesStatus {
            self.get_buf().get_u8_at(self.offset + 37).into()
        }

        /// CONSTANT 
        /// characterEncoding: 'US-ASCII'
        #[inline]
        pub fn market_id(&self) -> &'static [u8] {
            b"MOEX"
        }

        /// REQUIRED enum
        #[inline]
        pub fn market_segment_id(&self) -> MarketSegmentID {
            self.get_buf().get_u8_at(self.offset + 38).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn trad_ses_event(&self) -> TradSesEvent {
            self.get_buf().get_u8_at(self.offset + 39).into()
        }

    }

} // end decoder

