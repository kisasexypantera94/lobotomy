use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const SBE_BLOCK_LENGTH: u16 = 16;
pub const SBE_TEMPLATE_ID: u16 = 17;
pub const SBE_SCHEMA_ID: u16 = 19780;
pub const SBE_SCHEMA_VERSION: u16 = 3;
pub const SBE_SEMANTIC_VERSION: &str = "FIX5SP2";

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct OrderBookSnapshotEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for OrderBookSnapshotEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for OrderBookSnapshotEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> OrderBookSnapshotEncoder<'a> {
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

        /// primitive field 'LastMsgSeqNumProcessed'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 4
        /// - encodedLength: 4
        #[inline]
        pub fn last_msg_seq_num_processed(&mut self, value: u32) {
            let offset = self.offset + 4;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'RptSeq'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 4
        #[inline]
        pub fn rpt_seq(&mut self, value: u32) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'ExchangeTradingSessionID'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 12
        /// - encodedLength: 4
        #[inline]
        pub fn exchange_trading_session_id(&mut self, value: u32) {
            let offset = self.offset + 12;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// GROUP ENCODER
        #[inline]
        pub fn no_md_entries_encoder(self, count: u8, no_md_entries_encoder: NoMDEntriesEncoder<Self>) -> NoMDEntriesEncoder<Self> {
            no_md_entries_encoder.wrap(self, count)
        }

    }

    #[derive(Debug, Default)]
    pub struct NoMDEntriesEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for NoMDEntriesEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for NoMDEntriesEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoMDEntriesEncoder<P> where P: Encoder<'a> + Default {
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
            57
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

        /// primitive field 'TransactTime'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 8
        #[inline]
        pub fn transact_time(&mut self, value: u64) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn md_entry_px_encoder(self) -> Decimal5NULLEncoder<Self> {
            let offset = self.offset + 16;
            Decimal5NULLEncoder::default().wrap(self, offset)
        }

        /// primitive field 'MDEntrySize'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 24
        /// - encodedLength: 8
        #[inline]
        pub fn md_entry_size(&mut self, value: i64) {
            let offset = self.offset + 24;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'TradeID'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 32
        /// - encodedLength: 8
        #[inline]
        pub fn trade_id(&mut self, value: i64) {
            let offset = self.offset + 32;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        #[inline]
        pub fn md_flags(&mut self, value: MDFlagsSet) {
            let offset = self.offset + 40;
            self.get_buf_mut().put_u64_at(offset, value.0)
        }

        #[inline]
        pub fn md_flags_2(&mut self, value: MDFlags2Set) {
            let offset = self.offset + 48;
            self.get_buf_mut().put_u64_at(offset, value.0)
        }

        /// REQUIRED enum
        #[inline]
        pub fn md_entry_type(&mut self, value: MDEntryType) {
            let offset = self.offset + 56;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

    }

} // end encoder

pub mod decoder {
    use super::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct OrderBookSnapshotDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for OrderBookSnapshotDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for OrderBookSnapshotDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> OrderBookSnapshotDecoder<'a> {
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

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn last_msg_seq_num_processed(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 4)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn rpt_seq(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 8)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn exchange_trading_session_id(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 12)
        }

        /// GROUP DECODER
        #[inline]
        pub fn no_md_entries_decoder(self) -> NoMDEntriesDecoder<Self> {
            let acting_version = self.acting_version;
            NoMDEntriesDecoder::default().wrap(self, acting_version as usize)
        }

    }

    #[derive(Debug, Default)]
    pub struct NoMDEntriesDecoder<P> {
        parent: Option<P>,
        block_length: usize,
        acting_version: usize,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for NoMDEntriesDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for NoMDEntriesDecoder<P> where P: Decoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> NoMDEntriesDecoder<P> where P: Decoder<'a> + Default {
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

        /// group token - Token{signal=BEGIN_GROUP, name='NoMDEntries', referencedName='null', description='Number of entries in Market Data message', packageName='null', id=268, version=0, deprecated=0, encodedLength=57, offset=16, componentTokenCount=61, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
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

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808' }
        #[inline]
        pub fn md_entry_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn transact_time(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 8)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn md_entry_px_decoder(self) -> Decimal5NULLDecoder<Self> {
            let offset = self.offset + 16;
            Decimal5NULLDecoder::default().wrap(self, offset)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808' }
        #[inline]
        pub fn md_entry_size(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 24);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808' }
        #[inline]
        pub fn trade_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 32);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        #[inline]
        pub fn md_flags(&self) -> MDFlagsSet {
            MDFlagsSet::new(self.get_buf().get_u64_at(self.offset + 40))
        }

        #[inline]
        pub fn md_flags_2(&self) -> MDFlags2Set {
            MDFlags2Set::new(self.get_buf().get_u64_at(self.offset + 48))
        }

        /// REQUIRED enum
        #[inline]
        pub fn md_entry_type(&self) -> MDEntryType {
            self.get_buf().get_u8_at(self.offset + 56).into()
        }

    }

} // end decoder

