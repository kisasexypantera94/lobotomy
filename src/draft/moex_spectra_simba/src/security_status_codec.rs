use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const SBE_BLOCK_LENGTH: u16 = 70;
pub const SBE_TEMPLATE_ID: u16 = 9;
pub const SBE_SCHEMA_ID: u16 = 19780;
pub const SBE_SCHEMA_VERSION: u16 = 3;
pub const SBE_SEMANTIC_VERSION: &str = "FIX5SP2";

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct SecurityStatusEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for SecurityStatusEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for SecurityStatusEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> SecurityStatusEncoder<'a> {
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

        /// primitive field 'SecurityID'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 4
        #[inline]
        pub fn security_id(&mut self, value: i32) {
            let offset = self.offset;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        // skipping CONSTANT SecurityIDSource

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

        /// REQUIRED enum
        #[inline]
        pub fn security_trading_status(&mut self, value: SecurityTradingStatus) {
            let offset = self.offset + 29;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn high_limit_px_encoder(self) -> Decimal5NULLEncoder<Self> {
            let offset = self.offset + 30;
            Decimal5NULLEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn low_limit_px_encoder(self) -> Decimal5NULLEncoder<Self> {
            let offset = self.offset + 38;
            Decimal5NULLEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn initial_margin_on_buy_encoder(self) -> Decimal2NULLEncoder<Self> {
            let offset = self.offset + 46;
            Decimal2NULLEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn initial_margin_on_sell_encoder(self) -> Decimal2NULLEncoder<Self> {
            let offset = self.offset + 54;
            Decimal2NULLEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn initial_margin_syntetic_encoder(self) -> Decimal2NULLEncoder<Self> {
            let offset = self.offset + 62;
            Decimal2NULLEncoder::default().wrap(self, offset)
        }

    }

} // end encoder

pub mod decoder {
    use super::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct SecurityStatusDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for SecurityStatusDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for SecurityStatusDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> SecurityStatusDecoder<'a> {
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
        pub fn security_id(&self) -> i32 {
            self.get_buf().get_i32_at(self.offset)
        }

        /// CONSTANT 
        /// characterEncoding: 'US-ASCII'
        #[inline]
        pub fn security_id_source(&self) -> &'static [u8] {
            b"56"
        }

        #[inline]
        pub fn symbol(&self) -> [u8; 25] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 4)
        }

        /// REQUIRED enum
        #[inline]
        pub fn security_trading_status(&self) -> SecurityTradingStatus {
            self.get_buf().get_u8_at(self.offset + 29).into()
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn high_limit_px_decoder(self) -> Decimal5NULLDecoder<Self> {
            let offset = self.offset + 30;
            Decimal5NULLDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn low_limit_px_decoder(self) -> Decimal5NULLDecoder<Self> {
            let offset = self.offset + 38;
            Decimal5NULLDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn initial_margin_on_buy_decoder(self) -> Decimal2NULLDecoder<Self> {
            let offset = self.offset + 46;
            Decimal2NULLDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn initial_margin_on_sell_decoder(self) -> Decimal2NULLDecoder<Self> {
            let offset = self.offset + 54;
            Decimal2NULLDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn initial_margin_syntetic_decoder(self) -> Decimal2NULLDecoder<Self> {
            let offset = self.offset + 62;
            Decimal2NULLDecoder::default().wrap(self, offset)
        }

    }

} // end decoder

