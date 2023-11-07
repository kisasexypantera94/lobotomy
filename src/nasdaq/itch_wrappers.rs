use crate::common::types::{Amount, Price, TickSized};

use itchy::Price4;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Price4Wrapper(pub Price4);

impl From<Price4Wrapper> for f64 {
    fn from(value: Price4Wrapper) -> Self {
        const PRICE4_SCALE: f64 = 0.0001;
        value.0.raw() as f64 * PRICE4_SCALE
    }
}

impl PartialOrd for Price4Wrapper {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.raw().cmp(&other.0.raw()))
    }
}

impl std::ops::Mul<f64> for Price4Wrapper {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Self::Output {
        Price4Wrapper(Price4::from((self.0.raw() as f64 * rhs) as u32))
    }
}

impl TickSized for Price4Wrapper {
    #[inline(always)]
    fn px_to_tick_idx(px: &Self, tick_size: &Self) -> usize {
        (px.0.raw() / tick_size.0.raw()) as usize
    }

    #[inline(always)]
    fn tick_idx_to_px(tick_idx: &usize, tick_size: &Self) -> Self {
        Price4Wrapper(Price4::from(*tick_idx as u32 * tick_size.0.raw()))
    }

    #[inline(always)]
    fn round_to_tick_size(val: &Self, tick_size: &Self) -> Self {
        Price4Wrapper(Price4::from(
            val.0.raw() / tick_size.0.raw() * tick_size.0.raw(),
        ))
    }
}

impl Default for Price4Wrapper {
    fn default() -> Self {
        Price4Wrapper(Price4::from(0))
    }
}

impl Price for Price4Wrapper {}

impl Amount for u32 {
    type Delta = i64;

    #[inline(always)]
    fn apply_delta(&self, delta: &i64) -> Self {
        (*self as i64 + delta) as u32
    }
}
