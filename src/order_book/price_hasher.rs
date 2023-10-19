use crate::common::{intrinsics::*, utils};

#[derive(Debug)]
pub struct PriceHasher {
    pub px_min: f64,
    pub px_max: f64,
    tick_size: f64,
}

impl PriceHasher {
    const GROWTH_FACTOR: f64 = 0.001;

    pub fn new(start_px: f64, tick_size: f64) -> Self {
        PriceHasher {
            px_min: start_px,
            px_max: start_px,
            tick_size,
        }
    }

    pub fn hash(&mut self, px: f64) -> (usize, usize) {
        let tick_idx = self.px_to_tick_idx(px);
        let tick_idx_min = self.px_to_tick_idx(self.px_min);
        let tick_idx_max = self.px_to_tick_idx(self.px_max);

        // Case 1: px_min <= px <= px_max
        // Just extract the index, range remains the same, no need to shift
        if likely(tick_idx >= tick_idx_min && tick_idx <= tick_idx_max) {
            return (tick_idx - tick_idx_min, 0);
        }

        // Case 2: px < px_min
        // Range becomes [px_min * scale, px_max] and we have to shift to the right
        if tick_idx < tick_idx_min {
            let new_px_min = self.round_to_tick_size(px * (1.0 - Self::GROWTH_FACTOR));
            let new_tick_idx_min = self.px_to_tick_idx(new_px_min);
            self.px_min = new_px_min;

            return (tick_idx - new_tick_idx_min, tick_idx_min - new_tick_idx_min);
        }

        // Case 3: px_max < px
        // Range becomes [px_min, px_max * scale], no need to shift
        let new_px_max = self.round_to_tick_size(px * (1.0 + Self::GROWTH_FACTOR));
        self.px_max = new_px_max;

        (tick_idx - tick_idx_min, 0)
    }

    pub fn try_hash(&self, px: f64) -> Option<usize> {
        let tick_idx = self.px_to_tick_idx(px);
        let tick_idx_min = self.px_to_tick_idx(self.px_min);
        let tick_idx_max = self.px_to_tick_idx(self.px_max);

        if likely(tick_idx >= tick_idx_min && tick_idx <= tick_idx_max) {
            return Some(tick_idx - tick_idx_min);
        }

        None
    }

    pub fn idx_to_px(&self, idx: usize) -> f64 {
        (idx + self.px_to_tick_idx(self.px_min)) as f64 * self.tick_size
    }

    fn px_to_tick_idx(&self, px: f64) -> usize {
        utils::px_to_tick_idx(px, self.tick_size)
    }

    fn round_to_tick_size(&self, val: f64) -> f64 {
        utils::round_to_tick_size(val, self.tick_size)
    }
}
