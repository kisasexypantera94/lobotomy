use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const SBE_BLOCK_LENGTH: u16 = 250;
pub const SBE_TEMPLATE_ID: u16 = 12;
pub const SBE_SCHEMA_ID: u16 = 19780;
pub const SBE_SCHEMA_VERSION: u16 = 3;
pub const SBE_SEMANTIC_VERSION: &str = "FIX5SP2";

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct SecurityDefinitionEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for SecurityDefinitionEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for SecurityDefinitionEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> SecurityDefinitionEncoder<'a> {
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

        /// primitive field 'TotNumReports'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 4
        #[inline]
        pub fn tot_num_reports(&mut self, value: u32) {
            let offset = self.offset;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive array field 'Symbol'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 4
        /// - encodedLength: 25
        /// - version: 0
        #[inline]
        pub fn symbol(&mut self, value: [u8; 25]) {
            let offset = self.offset + 4;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive field 'SecurityID'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 29
        /// - encodedLength: 4
        #[inline]
        pub fn security_id(&mut self, value: i32) {
            let offset = self.offset + 29;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        // skipping CONSTANT SecurityIDSource

        /// primitive array field 'SecurityAltID'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 33
        /// - encodedLength: 25
        /// - version: 0
        #[inline]
        pub fn security_alt_id(&mut self, value: [u8; 25]) {
            let offset = self.offset + 33;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn security_alt_id_source(&mut self, value: SecurityAltIDSource) {
            let offset = self.offset + 58;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive array field 'SecurityType'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 59
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn security_type(&mut self, value: [u8; 4]) {
            let offset = self.offset + 59;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive array field 'CFICode'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 63
        /// - encodedLength: 6
        /// - version: 0
        #[inline]
        pub fn cfi_code(&mut self, value: [u8; 6]) {
            let offset = self.offset + 63;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn strike_price_encoder(self) -> Decimal5NULLEncoder<Self> {
            let offset = self.offset + 69;
            Decimal5NULLEncoder::default().wrap(self, offset)
        }

        /// primitive field 'ContractMultiplier'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 77
        /// - encodedLength: 4
        #[inline]
        pub fn contract_multiplier(&mut self, value: i32) {
            let offset = self.offset + 77;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn security_trading_status(&mut self, value: SecurityTradingStatus) {
            let offset = self.offset + 81;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive array field 'Currency'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 82
        /// - encodedLength: 3
        /// - version: 0
        #[inline]
        pub fn currency(&mut self, value: [u8; 3]) {
            let offset = self.offset + 82;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        // skipping CONSTANT MarketID

        /// REQUIRED enum
        #[inline]
        pub fn market_segment_id(&mut self, value: MarketSegmentID) {
            let offset = self.offset + 85;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn trading_session_id(&mut self, value: TradingSessionID) {
            let offset = self.offset + 86;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'ExchangeTradingSessionID'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 87
        /// - encodedLength: 4
        #[inline]
        pub fn exchange_trading_session_id(&mut self, value: i32) {
            let offset = self.offset + 87;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn volatility_encoder(self) -> Decimal5NULLEncoder<Self> {
            let offset = self.offset + 91;
            Decimal5NULLEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn high_limit_px_encoder(self) -> Decimal5NULLEncoder<Self> {
            let offset = self.offset + 99;
            Decimal5NULLEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn low_limit_px_encoder(self) -> Decimal5NULLEncoder<Self> {
            let offset = self.offset + 107;
            Decimal5NULLEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn min_price_increment_encoder(self) -> Decimal5NULLEncoder<Self> {
            let offset = self.offset + 115;
            Decimal5NULLEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn min_price_increment_amount_encoder(self) -> Decimal5NULLEncoder<Self> {
            let offset = self.offset + 123;
            Decimal5NULLEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn initial_margin_on_buy_encoder(self) -> Decimal2NULLEncoder<Self> {
            let offset = self.offset + 131;
            Decimal2NULLEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn initial_margin_on_sell_encoder(self) -> Decimal2NULLEncoder<Self> {
            let offset = self.offset + 139;
            Decimal2NULLEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn initial_margin_syntetic_encoder(self) -> Decimal2NULLEncoder<Self> {
            let offset = self.offset + 147;
            Decimal2NULLEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn theor_price_encoder(self) -> Decimal5NULLEncoder<Self> {
            let offset = self.offset + 155;
            Decimal5NULLEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn theor_price_limit_encoder(self) -> Decimal5NULLEncoder<Self> {
            let offset = self.offset + 163;
            Decimal5NULLEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn underlying_qty_encoder(self) -> Decimal5NULLEncoder<Self> {
            let offset = self.offset + 171;
            Decimal5NULLEncoder::default().wrap(self, offset)
        }

        /// primitive array field 'UnderlyingCurrency'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 179
        /// - encodedLength: 3
        /// - version: 0
        #[inline]
        pub fn underlying_currency(&mut self, value: [u8; 3]) {
            let offset = self.offset + 179;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive field 'MaturityDate'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 182
        /// - encodedLength: 4
        #[inline]
        pub fn maturity_date(&mut self, value: u32) {
            let offset = self.offset + 182;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'MaturityTime'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 186
        /// - encodedLength: 4
        #[inline]
        pub fn maturity_time(&mut self, value: u32) {
            let offset = self.offset + 186;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        #[inline]
        pub fn flags(&mut self, value: FlagsSet) {
            let offset = self.offset + 190;
            self.get_buf_mut().put_u64_at(offset, value.0)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn min_price_increment_amount_curr_encoder(self) -> Decimal5NULLEncoder<Self> {
            let offset = self.offset + 198;
            Decimal5NULLEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn settl_price_open_encoder(self) -> Decimal5NULLEncoder<Self> {
            let offset = self.offset + 206;
            Decimal5NULLEncoder::default().wrap(self, offset)
        }

        /// primitive array field 'ValuationMethod'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 214
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn valuation_method(&mut self, value: [u8; 4]) {
            let offset = self.offset + 214;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive field 'RiskFreeRate'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 218
        /// - encodedLength: 8
        #[inline]
        pub fn risk_free_rate(&mut self, value: f64) {
            let offset = self.offset + 218;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// primitive field 'FixedSpotDiscount'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 226
        /// - encodedLength: 8
        #[inline]
        pub fn fixed_spot_discount(&mut self, value: f64) {
            let offset = self.offset + 226;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// primitive field 'ProjectedSpotDiscount'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 234
        /// - encodedLength: 8
        #[inline]
        pub fn projected_spot_discount(&mut self, value: f64) {
            let offset = self.offset + 234;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// primitive array field 'SettlCurrency'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 242
        /// - encodedLength: 3
        /// - version: 0
        #[inline]
        pub fn settl_currency(&mut self, value: [u8; 3]) {
            let offset = self.offset + 242;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn negative_prices(&mut self, value: NegativePrices) {
            let offset = self.offset + 245;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'DerivativeContractMultiplier'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 246
        /// - encodedLength: 4
        #[inline]
        pub fn derivative_contract_multiplier(&mut self, value: i32) {
            let offset = self.offset + 246;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// GROUP ENCODER
        #[inline]
        pub fn no_md_feed_types_encoder(self, count: u8, no_md_feed_types_encoder: NoMDFeedTypesEncoder<Self>) -> NoMDFeedTypesEncoder<Self> {
            no_md_feed_types_encoder.wrap(self, count)
        }

        /// GROUP ENCODER
        #[inline]
        pub fn no_underlyings_encoder(self, count: u8, no_underlyings_encoder: NoUnderlyingsEncoder<Self>) -> NoUnderlyingsEncoder<Self> {
            no_underlyings_encoder.wrap(self, count)
        }

        /// GROUP ENCODER
        #[inline]
        pub fn no_legs_encoder(self, count: u8, no_legs_encoder: NoLegsEncoder<Self>) -> NoLegsEncoder<Self> {
            no_legs_encoder.wrap(self, count)
        }

        /// GROUP ENCODER
        #[inline]
        pub fn no_instr_attrib_encoder(self, count: u8, no_instr_attrib_encoder: NoInstrAttribEncoder<Self>) -> NoInstrAttribEncoder<Self> {
            no_instr_attrib_encoder.wrap(self, count)
        }

        /// GROUP ENCODER
        #[inline]
        pub fn no_events_encoder(self, count: u8, no_events_encoder: NoEventsEncoder<Self>) -> NoEventsEncoder<Self> {
            no_events_encoder.wrap(self, count)
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn security_desc(&mut self, value: &str) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 2 + data_length);
            self.get_buf_mut().put_u16_at(limit, data_length as u16);
            self.get_buf_mut().put_slice_at(limit + 2, value.as_bytes());
        }

        /// VAR_DATA ENCODER - character encoding: 'US-ASCII'
        #[inline]
        pub fn quotation_list(&mut self, value: &[u8]) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 2 + data_length);
            self.get_buf_mut().put_u16_at(limit, data_length as u16);
            self.get_buf_mut().put_slice_at(limit + 2, value);
        }

    }

    #[derive(Debug, Default)]
    pub struct NoMDFeedTypesEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for NoMDFeedTypesEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for NoMDFeedTypesEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoMDFeedTypesEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        pub fn wrap(
            mut self,
            mut parent: P,
            count: u8,
        ) -> Self {
            let initial_limit = parent.get_limit();
            parent.set_limit(initial_limit + 3);
            parent.get_buf_mut().put_u16_at(initial_limit, Self::block_length());
            parent.get_buf_mut().put_u8_at(initial_limit + 2, count);
            self.parent = Some(parent);
            self.count = count;
            self.index = usize::MAX;
            self.offset = usize::MAX;
            self.initial_limit = initial_limit;
            self
        }

        #[inline]
        pub fn block_length() -> u16 {
            33
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// will return Some(current index) when successful otherwise None
        #[inline]
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + Self::block_length() as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// primitive array field 'MDFeedType'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 25
        /// - version: 0
        #[inline]
        pub fn md_feed_type(&mut self, value: [u8; 25]) {
            let offset = self.offset;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive field 'MarketDepth'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 25
        /// - encodedLength: 4
        #[inline]
        pub fn market_depth(&mut self, value: u32) {
            let offset = self.offset + 25;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'MDBookType'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 29
        /// - encodedLength: 4
        #[inline]
        pub fn md_book_type(&mut self, value: u32) {
            let offset = self.offset + 29;
            self.get_buf_mut().put_u32_at(offset, value);
        }

    }

    #[derive(Debug, Default)]
    pub struct NoUnderlyingsEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for NoUnderlyingsEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for NoUnderlyingsEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoUnderlyingsEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        pub fn wrap(
            mut self,
            mut parent: P,
            count: u8,
        ) -> Self {
            let initial_limit = parent.get_limit();
            parent.set_limit(initial_limit + 3);
            parent.get_buf_mut().put_u16_at(initial_limit, Self::block_length());
            parent.get_buf_mut().put_u8_at(initial_limit + 2, count);
            self.parent = Some(parent);
            self.count = count;
            self.index = usize::MAX;
            self.offset = usize::MAX;
            self.initial_limit = initial_limit;
            self
        }

        #[inline]
        pub fn block_length() -> u16 {
            37
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// will return Some(current index) when successful otherwise None
        #[inline]
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + Self::block_length() as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// primitive array field 'UnderlyingSymbol'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 25
        /// - version: 0
        #[inline]
        pub fn underlying_symbol(&mut self, value: [u8; 25]) {
            let offset = self.offset;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive array field 'UnderlyingBoard'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 25
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn underlying_board(&mut self, value: [u8; 4]) {
            let offset = self.offset + 25;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive field 'UnderlyingSecurityID'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 29
        /// - encodedLength: 4
        #[inline]
        pub fn underlying_security_id(&mut self, value: i32) {
            let offset = self.offset + 29;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive field 'UnderlyingFutureID'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 33
        /// - encodedLength: 4
        #[inline]
        pub fn underlying_future_id(&mut self, value: i32) {
            let offset = self.offset + 33;
            self.get_buf_mut().put_i32_at(offset, value);
        }

    }

    #[derive(Debug, Default)]
    pub struct NoLegsEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for NoLegsEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for NoLegsEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoLegsEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        pub fn wrap(
            mut self,
            mut parent: P,
            count: u8,
        ) -> Self {
            let initial_limit = parent.get_limit();
            parent.set_limit(initial_limit + 3);
            parent.get_buf_mut().put_u16_at(initial_limit, Self::block_length());
            parent.get_buf_mut().put_u8_at(initial_limit + 2, count);
            self.parent = Some(parent);
            self.count = count;
            self.index = usize::MAX;
            self.offset = usize::MAX;
            self.initial_limit = initial_limit;
            self
        }

        #[inline]
        pub fn block_length() -> u16 {
            33
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// will return Some(current index) when successful otherwise None
        #[inline]
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + Self::block_length() as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// primitive array field 'LegSymbol'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 25
        /// - version: 0
        #[inline]
        pub fn leg_symbol(&mut self, value: [u8; 25]) {
            let offset = self.offset;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive field 'LegSecurityID'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 25
        /// - encodedLength: 4
        #[inline]
        pub fn leg_security_id(&mut self, value: i32) {
            let offset = self.offset + 25;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive field 'LegRatioQty'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 29
        /// - encodedLength: 4
        #[inline]
        pub fn leg_ratio_qty(&mut self, value: i32) {
            let offset = self.offset + 29;
            self.get_buf_mut().put_i32_at(offset, value);
        }

    }

    #[derive(Debug, Default)]
    pub struct NoInstrAttribEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for NoInstrAttribEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for NoInstrAttribEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoInstrAttribEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        pub fn wrap(
            mut self,
            mut parent: P,
            count: u8,
        ) -> Self {
            let initial_limit = parent.get_limit();
            parent.set_limit(initial_limit + 3);
            parent.get_buf_mut().put_u16_at(initial_limit, Self::block_length());
            parent.get_buf_mut().put_u8_at(initial_limit + 2, count);
            self.parent = Some(parent);
            self.count = count;
            self.index = usize::MAX;
            self.offset = usize::MAX;
            self.initial_limit = initial_limit;
            self
        }

        #[inline]
        pub fn block_length() -> u16 {
            35
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// will return Some(current index) when successful otherwise None
        #[inline]
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + Self::block_length() as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// primitive field 'InstrAttribType'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 4
        #[inline]
        pub fn instr_attrib_type(&mut self, value: i32) {
            let offset = self.offset;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive array field 'InstrAttribValue'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 4
        /// - encodedLength: 31
        /// - version: 0
        #[inline]
        pub fn instr_attrib_value(&mut self, value: [u8; 31]) {
            let offset = self.offset + 4;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

    }

    #[derive(Debug, Default)]
    pub struct NoEventsEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for NoEventsEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for NoEventsEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoEventsEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        pub fn wrap(
            mut self,
            mut parent: P,
            count: u8,
        ) -> Self {
            let initial_limit = parent.get_limit();
            parent.set_limit(initial_limit + 3);
            parent.get_buf_mut().put_u16_at(initial_limit, Self::block_length());
            parent.get_buf_mut().put_u8_at(initial_limit + 2, count);
            self.parent = Some(parent);
            self.count = count;
            self.index = usize::MAX;
            self.offset = usize::MAX;
            self.initial_limit = initial_limit;
            self
        }

        #[inline]
        pub fn block_length() -> u16 {
            16
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// will return Some(current index) when successful otherwise None
        #[inline]
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + Self::block_length() as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// primitive field 'EventType'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 4
        #[inline]
        pub fn event_type(&mut self, value: i32) {
            let offset = self.offset;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive field 'EventDate'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 4
        /// - encodedLength: 4
        #[inline]
        pub fn event_date(&mut self, value: u32) {
            let offset = self.offset + 4;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'EventTime'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 8
        #[inline]
        pub fn event_time(&mut self, value: u64) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u64_at(offset, value);
        }

    }

} // end encoder

pub mod decoder {
    use super::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct SecurityDefinitionDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for SecurityDefinitionDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for SecurityDefinitionDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> SecurityDefinitionDecoder<'a> {
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
        pub fn tot_num_reports(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset)
        }

        #[inline]
        pub fn symbol(&self) -> [u8; 25] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 4)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn security_id(&self) -> i32 {
            self.get_buf().get_i32_at(self.offset + 29)
        }

        /// CONSTANT 
        /// characterEncoding: 'US-ASCII'
        #[inline]
        pub fn security_id_source(&self) -> &'static [u8] {
            b"56"
        }

        #[inline]
        pub fn security_alt_id(&self) -> [u8; 25] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 33)
        }

        /// REQUIRED enum
        #[inline]
        pub fn security_alt_id_source(&self) -> SecurityAltIDSource {
            self.get_buf().get_u8_at(self.offset + 58).into()
        }

        #[inline]
        pub fn security_type(&self) -> [u8; 4] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 59)
        }

        #[inline]
        pub fn cfi_code(&self) -> [u8; 6] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 63)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn strike_price_decoder(self) -> Decimal5NULLDecoder<Self> {
            let offset = self.offset + 69;
            Decimal5NULLDecoder::default().wrap(self, offset)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-2147483648' }
        #[inline]
        pub fn contract_multiplier(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 77);
            if value == -2147483648_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn security_trading_status(&self) -> SecurityTradingStatus {
            self.get_buf().get_u8_at(self.offset + 81).into()
        }

        #[inline]
        pub fn currency(&self) -> [u8; 3] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 82)
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
            self.get_buf().get_u8_at(self.offset + 85).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn trading_session_id(&self) -> TradingSessionID {
            self.get_buf().get_u8_at(self.offset + 86).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-2147483648' }
        #[inline]
        pub fn exchange_trading_session_id(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 87);
            if value == -2147483648_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn volatility_decoder(self) -> Decimal5NULLDecoder<Self> {
            let offset = self.offset + 91;
            Decimal5NULLDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn high_limit_px_decoder(self) -> Decimal5NULLDecoder<Self> {
            let offset = self.offset + 99;
            Decimal5NULLDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn low_limit_px_decoder(self) -> Decimal5NULLDecoder<Self> {
            let offset = self.offset + 107;
            Decimal5NULLDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn min_price_increment_decoder(self) -> Decimal5NULLDecoder<Self> {
            let offset = self.offset + 115;
            Decimal5NULLDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn min_price_increment_amount_decoder(self) -> Decimal5NULLDecoder<Self> {
            let offset = self.offset + 123;
            Decimal5NULLDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn initial_margin_on_buy_decoder(self) -> Decimal2NULLDecoder<Self> {
            let offset = self.offset + 131;
            Decimal2NULLDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn initial_margin_on_sell_decoder(self) -> Decimal2NULLDecoder<Self> {
            let offset = self.offset + 139;
            Decimal2NULLDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn initial_margin_syntetic_decoder(self) -> Decimal2NULLDecoder<Self> {
            let offset = self.offset + 147;
            Decimal2NULLDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn theor_price_decoder(self) -> Decimal5NULLDecoder<Self> {
            let offset = self.offset + 155;
            Decimal5NULLDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn theor_price_limit_decoder(self) -> Decimal5NULLDecoder<Self> {
            let offset = self.offset + 163;
            Decimal5NULLDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn underlying_qty_decoder(self) -> Decimal5NULLDecoder<Self> {
            let offset = self.offset + 171;
            Decimal5NULLDecoder::default().wrap(self, offset)
        }

        #[inline]
        pub fn underlying_currency(&self) -> [u8; 3] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 179)
        }

        /// primitive field - 'OPTIONAL' { null_value: '4294967295' }
        #[inline]
        pub fn maturity_date(&self) -> Option<u32> {
            let value = self.get_buf().get_u32_at(self.offset + 182);
            if value == 0xffffffff_u32 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '4294967295' }
        #[inline]
        pub fn maturity_time(&self) -> Option<u32> {
            let value = self.get_buf().get_u32_at(self.offset + 186);
            if value == 0xffffffff_u32 {
                None
            } else {
                Some(value)
            }
        }

        #[inline]
        pub fn flags(&self) -> FlagsSet {
            FlagsSet::new(self.get_buf().get_u64_at(self.offset + 190))
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn min_price_increment_amount_curr_decoder(self) -> Decimal5NULLDecoder<Self> {
            let offset = self.offset + 198;
            Decimal5NULLDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn settl_price_open_decoder(self) -> Decimal5NULLDecoder<Self> {
            let offset = self.offset + 206;
            Decimal5NULLDecoder::default().wrap(self, offset)
        }

        #[inline]
        pub fn valuation_method(&self) -> [u8; 4] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 214)
        }

        /// primitive field - 'OPTIONAL' { null_value: 'NaN' }
        #[inline]
        pub fn risk_free_rate(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 218);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: 'NaN' }
        #[inline]
        pub fn fixed_spot_discount(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 226);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: 'NaN' }
        #[inline]
        pub fn projected_spot_discount(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 234);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        #[inline]
        pub fn settl_currency(&self) -> [u8; 3] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 242)
        }

        /// REQUIRED enum
        #[inline]
        pub fn negative_prices(&self) -> NegativePrices {
            self.get_buf().get_u8_at(self.offset + 245).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-2147483648' }
        #[inline]
        pub fn derivative_contract_multiplier(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 246);
            if value == -2147483648_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// GROUP DECODER
        #[inline]
        pub fn no_md_feed_types_decoder(self) -> NoMDFeedTypesDecoder<Self> {
            let acting_version = self.acting_version;
            NoMDFeedTypesDecoder::default().wrap(self, acting_version as usize)
        }

        /// GROUP DECODER
        #[inline]
        pub fn no_underlyings_decoder(self) -> NoUnderlyingsDecoder<Self> {
            let acting_version = self.acting_version;
            NoUnderlyingsDecoder::default().wrap(self, acting_version as usize)
        }

        /// GROUP DECODER
        #[inline]
        pub fn no_legs_decoder(self) -> NoLegsDecoder<Self> {
            let acting_version = self.acting_version;
            NoLegsDecoder::default().wrap(self, acting_version as usize)
        }

        /// GROUP DECODER
        #[inline]
        pub fn no_instr_attrib_decoder(self) -> NoInstrAttribDecoder<Self> {
            let acting_version = self.acting_version;
            NoInstrAttribDecoder::default().wrap(self, acting_version as usize)
        }

        /// GROUP DECODER
        #[inline]
        pub fn no_events_decoder(self) -> NoEventsDecoder<Self> {
            let acting_version = self.acting_version;
            NoEventsDecoder::default().wrap(self, acting_version as usize)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn security_desc_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u16_at(offset) as usize;
            self.set_limit(offset + 2 + data_length);
            (offset + 2, data_length)
        }

        #[inline]
        pub fn security_desc_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'US-ASCII'
        #[inline]
        pub fn quotation_list_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u16_at(offset) as usize;
            self.set_limit(offset + 2 + data_length);
            (offset + 2, data_length)
        }

        #[inline]
        pub fn quotation_list_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

    }

    #[derive(Debug, Default)]
    pub struct NoMDFeedTypesDecoder<P> {
        parent: Option<P>,
        block_length: usize,
        acting_version: usize,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for NoMDFeedTypesDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for NoMDFeedTypesDecoder<P> where P: Decoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoMDFeedTypesDecoder<P> where P: Decoder<'a> + Default {
        pub fn wrap(
            mut self,
            mut parent: P,
            acting_version: usize,
        ) -> Self {
            let initial_offset = parent.get_limit();
            let block_length = parent.get_buf().get_u16_at(initial_offset) as usize;
            let count = parent.get_buf().get_u8_at(initial_offset + 2);
            parent.set_limit(initial_offset + 3);
            self.parent = Some(parent);
            self.block_length = block_length;
            self.acting_version = acting_version;
            self.count = count;
            self.index = usize::MAX;
            self.offset = 0;
            self
        }

        /// group token - Token{signal=BEGIN_GROUP, name='NoMDFeedTypes', referencedName='null', description='Number of feed types', packageName='null', id=1141, version=0, deprecated=0, encodedLength=33, offset=250, componentTokenCount=15, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        #[inline]
        pub fn count(&self) -> u8 {
            self.count
        }

        /// will return Some(current index) when successful otherwise None
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                 return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + self.block_length);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        #[inline]
        pub fn md_feed_type(&self) -> [u8; 25] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset)
        }

        /// primitive field - 'OPTIONAL' { null_value: '4294967295' }
        #[inline]
        pub fn market_depth(&self) -> Option<u32> {
            let value = self.get_buf().get_u32_at(self.offset + 25);
            if value == 0xffffffff_u32 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '4294967295' }
        #[inline]
        pub fn md_book_type(&self) -> Option<u32> {
            let value = self.get_buf().get_u32_at(self.offset + 29);
            if value == 0xffffffff_u32 {
                None
            } else {
                Some(value)
            }
        }

    }

    #[derive(Debug, Default)]
    pub struct NoUnderlyingsDecoder<P> {
        parent: Option<P>,
        block_length: usize,
        acting_version: usize,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for NoUnderlyingsDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for NoUnderlyingsDecoder<P> where P: Decoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoUnderlyingsDecoder<P> where P: Decoder<'a> + Default {
        pub fn wrap(
            mut self,
            mut parent: P,
            acting_version: usize,
        ) -> Self {
            let initial_offset = parent.get_limit();
            let block_length = parent.get_buf().get_u16_at(initial_offset) as usize;
            let count = parent.get_buf().get_u8_at(initial_offset + 2);
            parent.set_limit(initial_offset + 3);
            self.parent = Some(parent);
            self.block_length = block_length;
            self.acting_version = acting_version;
            self.count = count;
            self.index = usize::MAX;
            self.offset = 0;
            self
        }

        /// group token - Token{signal=BEGIN_GROUP, name='NoUnderlyings', referencedName='null', description='Number of underlyings', packageName='null', id=711, version=0, deprecated=0, encodedLength=37, offset=-1, componentTokenCount=18, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        #[inline]
        pub fn count(&self) -> u8 {
            self.count
        }

        /// will return Some(current index) when successful otherwise None
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                 return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + self.block_length);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        #[inline]
        pub fn underlying_symbol(&self) -> [u8; 25] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset)
        }

        #[inline]
        pub fn underlying_board(&self) -> [u8; 4] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 25)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-2147483648' }
        #[inline]
        pub fn underlying_security_id(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 29);
            if value == -2147483648_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-2147483648' }
        #[inline]
        pub fn underlying_future_id(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 33);
            if value == -2147483648_i32 {
                None
            } else {
                Some(value)
            }
        }

    }

    #[derive(Debug, Default)]
    pub struct NoLegsDecoder<P> {
        parent: Option<P>,
        block_length: usize,
        acting_version: usize,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for NoLegsDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for NoLegsDecoder<P> where P: Decoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoLegsDecoder<P> where P: Decoder<'a> + Default {
        pub fn wrap(
            mut self,
            mut parent: P,
            acting_version: usize,
        ) -> Self {
            let initial_offset = parent.get_limit();
            let block_length = parent.get_buf().get_u16_at(initial_offset) as usize;
            let count = parent.get_buf().get_u8_at(initial_offset + 2);
            parent.set_limit(initial_offset + 3);
            self.parent = Some(parent);
            self.block_length = block_length;
            self.acting_version = acting_version;
            self.count = count;
            self.index = usize::MAX;
            self.offset = 0;
            self
        }

        /// group token - Token{signal=BEGIN_GROUP, name='NoLegs', referencedName='null', description='Nymber of legs', packageName='null', id=555, version=0, deprecated=0, encodedLength=33, offset=-1, componentTokenCount=15, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        #[inline]
        pub fn count(&self) -> u8 {
            self.count
        }

        /// will return Some(current index) when successful otherwise None
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                 return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + self.block_length);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        #[inline]
        pub fn leg_symbol(&self) -> [u8; 25] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn leg_security_id(&self) -> i32 {
            self.get_buf().get_i32_at(self.offset + 25)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn leg_ratio_qty(&self) -> i32 {
            self.get_buf().get_i32_at(self.offset + 29)
        }

    }

    #[derive(Debug, Default)]
    pub struct NoInstrAttribDecoder<P> {
        parent: Option<P>,
        block_length: usize,
        acting_version: usize,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for NoInstrAttribDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for NoInstrAttribDecoder<P> where P: Decoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoInstrAttribDecoder<P> where P: Decoder<'a> + Default {
        pub fn wrap(
            mut self,
            mut parent: P,
            acting_version: usize,
        ) -> Self {
            let initial_offset = parent.get_limit();
            let block_length = parent.get_buf().get_u16_at(initial_offset) as usize;
            let count = parent.get_buf().get_u8_at(initial_offset + 2);
            parent.set_limit(initial_offset + 3);
            self.parent = Some(parent);
            self.block_length = block_length;
            self.acting_version = acting_version;
            self.count = count;
            self.index = usize::MAX;
            self.offset = 0;
            self
        }

        /// group token - Token{signal=BEGIN_GROUP, name='NoInstrAttrib', referencedName='null', description='Number of attributes', packageName='null', id=870, version=0, deprecated=0, encodedLength=35, offset=-1, componentTokenCount=12, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        #[inline]
        pub fn count(&self) -> u8 {
            self.count
        }

        /// will return Some(current index) when successful otherwise None
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                 return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + self.block_length);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn instr_attrib_type(&self) -> i32 {
            self.get_buf().get_i32_at(self.offset)
        }

        #[inline]
        pub fn instr_attrib_value(&self) -> [u8; 31] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 4)
        }

    }

    #[derive(Debug, Default)]
    pub struct NoEventsDecoder<P> {
        parent: Option<P>,
        block_length: usize,
        acting_version: usize,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for NoEventsDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for NoEventsDecoder<P> where P: Decoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoEventsDecoder<P> where P: Decoder<'a> + Default {
        pub fn wrap(
            mut self,
            mut parent: P,
            acting_version: usize,
        ) -> Self {
            let initial_offset = parent.get_limit();
            let block_length = parent.get_buf().get_u16_at(initial_offset) as usize;
            let count = parent.get_buf().get_u8_at(initial_offset + 2);
            parent.set_limit(initial_offset + 3);
            self.parent = Some(parent);
            self.block_length = block_length;
            self.acting_version = acting_version;
            self.count = count;
            self.index = usize::MAX;
            self.offset = 0;
            self
        }

        /// group token - Token{signal=BEGIN_GROUP, name='NoEvents', referencedName='null', description='Number of events', packageName='null', id=864, version=0, deprecated=0, encodedLength=16, offset=-1, componentTokenCount=15, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        #[inline]
        pub fn count(&self) -> u8 {
            self.count
        }

        /// will return Some(current index) when successful otherwise None
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                 return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + self.block_length);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn event_type(&self) -> i32 {
            self.get_buf().get_i32_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn event_date(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 4)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn event_time(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 8)
        }

    }

} // end decoder

