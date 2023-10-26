use crate::common::intrinsics::*;

use super::price_hasher::PriceHasher;

#[derive(Debug, Clone, Copy, Default)]
pub struct PriceLevel {
    pub amt: f64,
}

#[derive(Debug)]
pub struct PriceMap {
    px_hasher: PriceHasher,
    levels: Vec<PriceLevel>,
}

impl PriceMap {
    pub fn new(start_px: f64, tick_size: f64) -> Self {
        PriceMap {
            px_hasher: PriceHasher::new(start_px, tick_size),
            levels: Vec::new(),
        }
    }

    /// May trigger rehashing
    pub fn get_mut(&mut self, px: f64) -> &mut PriceLevel {
        let (px_idx, shift) = self.px_hasher.hash(px);

        if unlikely(px_idx >= self.levels.len()) {
            self.levels.resize(px_idx + 1, PriceLevel::default());
        }

        if likely(shift == 0) {
            return &mut self.levels[px_idx];
        }

        let mut new_levels = vec![PriceLevel::default(); shift + self.levels.len()];

        for (i, level) in self.levels.iter().enumerate() {
            new_levels[shift + i] = *level;
        }

        self.levels = new_levels;

        &mut self.levels[px_idx]
    }

    /// May not trigger rehashing
    pub fn get_immut(&self, px: f64) -> PriceLevel {
        match self.px_hasher.try_hash(px) {
            Some(px_idx) => self.levels[px_idx],
            None => PriceLevel::default(),
        }
    }

    pub fn next_px<const REVERSE: bool>(&self, px: f64) -> Option<f64> {
        let px_idx = match self.px_hasher.try_hash(px) {
            Some(px_idx) => px_idx,
            None => return None,
        };

        if REVERSE {
            for (idx, level) in self
                .levels
                .iter()
                .enumerate()
                .rev()
                .skip(self.levels.len() - px_idx)
            {
                if level.amt > 0.0 {
                    return Some(self.px_hasher.idx_to_px(idx));
                }
            }
        } else {
            for (idx, level) in self.levels.iter().enumerate().skip(px_idx + 1) {
                if level.amt > 0.0 {
                    return Some(self.px_hasher.idx_to_px(idx));
                }
            }
        }

        None
    }

    pub fn clear(&mut self) {
        self.levels.clear();
    }
}

impl PriceMap {
    pub fn top_levels<const N: usize, const REVERSE: bool>(
        &self,
    ) -> [Option<(f64, PriceLevel)>; N] {
        if REVERSE {
            self.collect_top(self.levels.iter().enumerate().rev())
        } else {
            self.collect_top(self.levels.iter().enumerate())
        }
    }

    fn collect_top<'a, const N: usize, Iter>(&self, iter: Iter) -> [Option<(f64, PriceLevel)>; N]
    where
        Iter: Iterator<Item = (usize, &'a PriceLevel)>,
    {
        let mut top = [None; N];
        let mut cur_idx = 0;

        for (idx, level) in iter {
            if level.amt > 0.0 {
                top[cur_idx] = Some((self.px_hasher.idx_to_px(idx), *level));

                cur_idx += 1;
                if cur_idx == N {
                    break;
                }
            }
        }

        top
    }
}
