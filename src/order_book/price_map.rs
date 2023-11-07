use super::price_hasher::PriceHasher;
use crate::common::{
    intrinsics::*,
    types::{Amount, Level, Price},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct PriceLevel<A> {
    pub amt: A,
}

#[derive(Debug, Clone)]
pub struct PriceMap<P: Price, A: Amount> {
    px_hasher: PriceHasher<P>,
    levels: Vec<PriceLevel<A>>,
}

impl<P: Price, A: Amount> PriceMap<P, A> {
    pub fn new(start_px: P, _end_px: Option<P>, tick_size: P) -> Self {
        PriceMap {
            px_hasher: PriceHasher::new(start_px, tick_size),
            levels: Vec::new(),
        }
    }

    /// May trigger rehashing
    #[inline(always)]
    pub fn get_mut(&mut self, px: P) -> &mut PriceLevel<A> {
        let (px_idx, shift) = self.px_hasher.hash(&px);

        if unlikely(px_idx >= self.levels.len()) {
            self.levels.resize(px_idx + 1, PriceLevel::default());
            log::debug!("Resize triggered: new_len=[{}]", self.levels.len());
        }

        if likely(shift == 0) {
            return &mut self.levels[px_idx];
        }

        let mut new_levels = vec![PriceLevel::default(); shift + self.levels.len()];

        for (i, level) in self.levels.iter().enumerate() {
            new_levels[shift + i] = *level;
        }

        self.levels = new_levels;
        log::debug!("Shift triggered: new_len=[{}]", self.levels.len());

        &mut self.levels[px_idx]
    }

    /// May not trigger rehashing
    #[inline(always)]
    pub fn get_immut(&self, px: P) -> PriceLevel<A> {
        match self.px_hasher.try_hash(&px) {
            Some(px_idx) => self.levels[px_idx],
            None => PriceLevel::default(),
        }
    }

    #[inline(always)]
    pub fn next_px<const REVERSE: bool>(&self, px: &P) -> Option<Level<P, A>> {
        let px_idx = match self.px_hasher.try_hash(&px) {
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
                if likely(!level.amt.is_zero()) {
                    return Some(Level {
                        px: self.px_hasher.idx_to_px(&idx),
                        amt: level.amt,
                    });
                }
            }
        } else {
            for (idx, level) in self.levels.iter().enumerate().skip(px_idx + 1) {
                if likely(!level.amt.is_zero()) {
                    return Some(Level {
                        px: self.px_hasher.idx_to_px(&idx),
                        amt: level.amt,
                    });
                }
            }
        }

        None
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.levels.clear();
    }
}

impl<'a, P: Price, A: Amount + 'a> PriceMap<P, A> {
    pub fn top_levels<const N: usize, const REVERSE: bool>(
        &self,
    ) -> [Option<(P, PriceLevel<A>)>; N] {
        if REVERSE {
            self.collect_top(self.levels.iter().enumerate().rev())
        } else {
            self.collect_top(self.levels.iter().enumerate())
        }
    }

    fn collect_top<const N: usize, Iter>(&self, iter: Iter) -> [Option<(P, PriceLevel<A>)>; N]
    where
        Iter: Iterator<Item = (usize, &'a PriceLevel<A>)>,
    {
        let mut top = [None; N];
        let mut cur_idx = 0;

        for (idx, level) in iter {
            if !level.amt.is_zero() {
                top[cur_idx] = Some((self.px_hasher.idx_to_px(&idx), *level));

                cur_idx += 1;
                if cur_idx == N {
                    break;
                }
            }
        }

        top
    }
}
