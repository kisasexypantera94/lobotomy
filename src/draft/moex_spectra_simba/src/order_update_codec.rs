use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const SBE_BLOCK_LENGTH: u16 = 50;
pub const SBE_TEMPLATE_ID: u16 = 15;
pub const SBE_SCHEMA_ID: u16 = 19780;
pub const SBE_SCHEMA_VERSION: u16 = 3;
pub const SBE_SEMANTIC_VERSION: &str = "FIX5SP2";

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct OrderUpdateEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for OrderUpdateEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for OrderUpdateEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> OrderUpdateEncoder<'a> {
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

        /// primitive field 'MDEntryID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 8
        #[inline]
        pub fn md_entry_id(&mut self, value: i64) {
            let offset = self.offset;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn md_entry_px_encoder(self) -> Decimal5Encoder<Self> {
            let offset = self.offset + 8;
            Decimal5Encoder::default().wrap(self, offset)
        }

        /// primitive field 'MDEntrySize'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 16
        /// - encodedLength: 8
        #[inline]
        pub fn md_entry_size(&mut self, value: i64) {
            let offset = self.offset + 16;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        #[inline]
        pub fn md_flags(&mut self, value: MDFlagsSet) {
            let offset = self.offset + 24;
            self.get_buf_mut().put_u64_at(offset, value.0)
        }

        #[inline]
        pub fn md_flags_2(&mut self, value: MDFlags2Set) {
            let offset = self.offset + 32;
            self.get_buf_mut().put_u64_at(offset, value.0)
        }

        /// primitive field 'SecurityID'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 40
        /// - encodedLength: 4
        #[inline]
        pub fn security_id(&mut self, value: i32) {
            let offset = self.offset + 40;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive field 'RptSeq'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 44
        /// - encodedLength: 4
        #[inline]
        pub fn rpt_seq(&mut self, value: u32) {
            let offset = self.offset + 44;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn md_update_action(&mut self, value: MDUpdateAction) {
            let offset = self.offset + 48;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn md_entry_type(&mut self, value: MDEntryType) {
            let offset = self.offset + 49;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

    }

} // end encoder

pub mod decoder {
    use super::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct OrderUpdateDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for OrderUpdateDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for OrderUpdateDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> OrderUpdateDecoder<'a> {
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
        pub fn md_entry_id(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn md_entry_px_decoder(self) -> Decimal5Decoder<Self> {
            let offset = self.offset + 8;
            Decimal5Decoder::default().wrap(self, offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn md_entry_size(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 16)
        }

        #[inline]
        pub fn md_flags(&self) -> MDFlagsSet {
            MDFlagsSet::new(self.get_buf().get_u64_at(self.offset + 24))
        }

        #[inline]
        pub fn md_flags_2(&self) -> MDFlags2Set {
            MDFlags2Set::new(self.get_buf().get_u64_at(self.offset + 32))
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn security_id(&self) -> i32 {
            self.get_buf().get_i32_at(self.offset + 40)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn rpt_seq(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 44)
        }

        /// REQUIRED enum
        #[inline]
        pub fn md_update_action(&self) -> MDUpdateAction {
            self.get_buf().get_u8_at(self.offset + 48).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn md_entry_type(&self) -> MDEntryType {
            self.get_buf().get_u8_at(self.offset + 49).into()
        }

    }

} // end decoder

