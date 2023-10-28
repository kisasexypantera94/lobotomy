use crate::common::{intrinsics::*, utils};

#[derive(Debug)]
pub struct PriceHasher {
    px_min: f64,
    tick_size: f64,
}

impl PriceHasher {
    const GROWTH_FACTOR: f64 = 0.1;

    pub fn new(start_px: f64, tick_size: f64) -> Self {
        PriceHasher {
            px_min: start_px,
            tick_size,
        }
    }

    #[inline(always)]
    pub fn hash(&mut self, px: f64) -> (usize, usize) {
        let tick_idx = self.px_to_tick_idx(px);
        let tick_idx_min = self.px_to_tick_idx(self.px_min);

        // Case 1: px_min <= px <= px_max
        // Just extract the index, range remains the same, no need to shift
        if likely(tick_idx >= tick_idx_min) {
            return (tick_idx - tick_idx_min, 0);
        }

        // Case 2: px < px_min
        // Range becomes [px_min * scale, px_max] and we have to shift to the right
        let new_px_min = self.round_to_tick_size(px * (1.0 - Self::GROWTH_FACTOR));
        let new_tick_idx_min = self.px_to_tick_idx(new_px_min);
        self.px_min = new_px_min;

        return (tick_idx - new_tick_idx_min, tick_idx_min - new_tick_idx_min);
    }

    #[inline(always)]
    pub fn try_hash(&self, px: f64) -> Option<usize> {
        let tick_idx = self.px_to_tick_idx(px);
        let tick_idx_min = self.px_to_tick_idx(self.px_min);

        if likely(tick_idx >= tick_idx_min) {
            return Some(tick_idx - tick_idx_min);
        }

        None
    }

    #[inline(always)]
    pub fn idx_to_px(&self, idx: usize) -> f64 {
        (idx + self.px_to_tick_idx(self.px_min)) as f64 * self.tick_size
    }

    #[inline(always)]
    fn px_to_tick_idx(&self, px: f64) -> usize {
        utils::px_to_tick_idx(px, self.tick_size)
    }

    #[inline(always)]
    fn round_to_tick_size(&self, val: f64) -> f64 {
        utils::round_to_tick_size(val, self.tick_size)
    }
}
