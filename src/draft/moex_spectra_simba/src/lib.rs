#![allow(warnings)]
#![forbid(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_camel_case_types)]
use ::core::convert::TryInto;

pub mod best_prices_codec;
pub mod decimal_2null_codec;
pub mod decimal_5_codec;
pub mod decimal_5null_codec;
pub mod discrete_auction_codec;
pub mod empty_book_codec;
pub mod flags_set;
pub mod group_size_codec;
pub mod heartbeat_codec;
pub mod logon_codec;
pub mod logout_codec;
pub mod market_data_request_codec;
pub mod market_segment_id;
pub mod md_entry_type;
pub mod md_flags_2_set;
pub mod md_flags_set;
pub mod md_update_action;
pub mod message_header_codec;
pub mod negative_prices;
pub mod order_book_snapshot_codec;
pub mod order_execution_codec;
pub mod order_update_codec;
pub mod security_alt_id_source;
pub mod security_definition_codec;
pub mod security_definition_update_report_codec;
pub mod security_status_codec;
pub mod security_trading_status;
pub mod sequence_reset_codec;
pub mod trad_ses_event;
pub mod trad_ses_status;
pub mod trading_session_id;
pub mod trading_session_status_codec;
pub mod utf_8_string_codec;
pub mod var_string_codec;

pub use best_prices_codec::*;
pub use decimal_2null_codec::*;
pub use decimal_5_codec::*;
pub use decimal_5null_codec::*;
pub use discrete_auction_codec::*;
pub use empty_book_codec::*;
pub use flags_set::*;
pub use group_size_codec::*;
pub use heartbeat_codec::*;
pub use logon_codec::*;
pub use logout_codec::*;
pub use market_data_request_codec::*;
pub use market_segment_id::*;
pub use md_entry_type::*;
pub use md_flags_2_set::*;
pub use md_flags_set::*;
pub use md_update_action::*;
pub use message_header_codec::*;
pub use negative_prices::*;
pub use order_book_snapshot_codec::*;
pub use order_execution_codec::*;
pub use order_update_codec::*;
pub use security_alt_id_source::*;
pub use security_definition_codec::*;
pub use security_definition_update_report_codec::*;
pub use security_status_codec::*;
pub use security_trading_status::*;
pub use sequence_reset_codec::*;
pub use trad_ses_event::*;
pub use trad_ses_status::*;
pub use trading_session_id::*;
pub use trading_session_status_codec::*;
pub use utf_8_string_codec::*;
pub use var_string_codec::*;

pub type SbeResult<T> = core::result::Result<T, SbeErr>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SbeErr {
    ParentNotSet,
}
impl core::fmt::Display for SbeErr {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for SbeErr {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub trait Writer<'a>: Sized {
    fn get_buf_mut(&mut self) -> &mut WriteBuf<'a>;
}

pub trait Encoder<'a>: Writer<'a> {
    fn get_limit(&self) -> usize;
    fn set_limit(&mut self, limit: usize);
}

pub trait Reader<'a>: Sized {
    fn get_buf(&self) -> &ReadBuf<'a>;
}

pub trait Decoder<'a>: Reader<'a> {
    fn get_limit(&self) -> usize;
    fn set_limit(&mut self, limit: usize);
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ReadBuf<'a> {
    data: &'a [u8],
}
impl<'a> Reader<'a> for ReadBuf<'a> {
    #[inline]
    fn get_buf(&self) -> &ReadBuf<'a> {
        self
    }
}
impl<'a> ReadBuf<'a> {
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    #[inline]
    fn get_bytes<const COUNT: usize>(slice: &[u8]) -> [u8; COUNT] {
        slice.try_into().expect("slice with incorrect length")
    }

    #[inline]
    pub(crate) fn get_bytes_at<const N: usize>(slice: &[u8], index: usize) -> [u8; N] {
        slice[index..index + N]
            .try_into()
            .expect("slice with incorrect length")
    }

    #[inline]
    pub fn get_u8_at(&self, index: usize) -> u8 {
        self.data[index]
    }

    #[inline]
    pub fn get_i8_at(&self, index: usize) -> i8 {
        i8::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i16_at(&self, index: usize) -> i16 {
        i16::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i32_at(&self, index: usize) -> i32 {
        i32::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i64_at(&self, index: usize) -> i64 {
        i64::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_u16_at(&self, index: usize) -> u16 {
        u16::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_u32_at(&self, index: usize) -> u32 {
        u32::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_u64_at(&self, index: usize) -> u64 {
        u64::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_f32_at(&self, index: usize) -> f32 {
        f32::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_f64_at(&self, index: usize) -> f64 {
        f64::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_slice_at(&self, index: usize, len: usize) -> &[u8] {
        &self.data[index..index + len]
    }
}

#[derive(Debug, Default)]
pub struct WriteBuf<'a> {
    data: &'a mut [u8],
}
impl<'a> WriteBuf<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data }
    }

    #[inline]
    pub fn put_bytes_at<const COUNT: usize>(&mut self, index: usize, bytes: [u8; COUNT]) -> usize {
        self.data[index..index + COUNT].copy_from_slice(&bytes);
        COUNT
    }

    #[inline]
    pub fn put_u8_at(&mut self, index: usize, value: u8) {
        self.data[index] = value;
    }

    #[inline]
    pub fn put_i8_at(&mut self, index: usize, value: i8) {
        self.put_bytes_at(index, i8::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i16_at(&mut self, index: usize, value: i16) {
        self.put_bytes_at(index, i16::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i32_at(&mut self, index: usize, value: i32) {
        self.put_bytes_at(index, i32::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i64_at(&mut self, index: usize, value: i64) {
        self.put_bytes_at(index, i64::to_le_bytes(value));
    }

    #[inline]
    pub fn put_u16_at(&mut self, index: usize, value: u16) {
        self.put_bytes_at(index, u16::to_le_bytes(value));
    }

    #[inline]
    pub fn put_u32_at(&mut self, index: usize, value: u32) {
        self.put_bytes_at(index, u32::to_le_bytes(value));
    }

    #[inline]
    pub fn put_u64_at(&mut self, index: usize, value: u64) {
        self.put_bytes_at(index, u64::to_le_bytes(value));
    }

    #[inline]
    pub fn put_f32_at(&mut self, index: usize, value: f32) {
        self.put_bytes_at(index, f32::to_le_bytes(value));
    }

    #[inline]
    pub fn put_f64_at(&mut self, index: usize, value: f64) {
        self.put_bytes_at(index, f64::to_le_bytes(value));
    }

    #[inline]
    pub fn put_slice_at(&mut self, index: usize, src: &[u8]) -> usize {
        let len = src.len();
        let dest = self.data.split_at_mut(index).1.split_at_mut(len).0;
        dest.clone_from_slice(src);
        len
    }
}
