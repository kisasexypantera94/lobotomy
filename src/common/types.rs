use num_traits::Zero;

use std::fmt::Debug;

#[derive(Default, Debug, Clone, Copy)]
pub struct Level<P, A> {
    pub px: P,
    pub amt: A,
}

#[derive(Debug, Clone, Copy)]
pub struct L2Delta<P, A: Amount> {
    pub px: P,
    pub amt_delta: <A as Amount>::Delta,
}

pub trait TickSized {
    fn px_to_tick_idx(px: &Self, tick_size: &Self) -> usize;
    fn tick_idx_to_px(tick_idx: &usize, tick_size: &Self) -> Self;
    fn round_to_tick_size(val: &Self, tick_size: &Self) -> Self;
}

impl TickSized for f64 {
    #[inline(always)]
    fn px_to_tick_idx(px: &Self, tick_size: &Self) -> usize {
        (px / tick_size).round() as usize
    }

    #[inline(always)]
    fn tick_idx_to_px(tick_idx: &usize, tick_size: &Self) -> Self {
        *tick_idx as Self * tick_size
    }

    #[inline(always)]
    fn round_to_tick_size(val: &Self, tick_size: &Self) -> f64 {
        let scale = 1.0 / tick_size;
        (val * scale + 0.5).floor() / scale
    }
}

pub trait Price:
    TickSized
    + Copy
    + std::ops::Mul<f64, Output = Self>
    + std::cmp::PartialOrd<Self>
    + Default
    + PartialEq
    + Debug
{
}

pub trait Amount: Copy + Zero + std::ops::AddAssign + Default + Debug {
    type Delta: Debug + Copy;

    fn apply_delta(&self, delta: &Self::Delta) -> Self;
}

impl Price for f64 {}

impl Amount for f64 {
    type Delta = f64;

    #[inline(always)]
    fn apply_delta(&self, delta: &Self::Delta) -> Self {
        self + delta
    }
}
