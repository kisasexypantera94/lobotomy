use super::PriceLevel;
use super::PriceMap;
use super::PriceTop;
use crate::common::intrinsics::*;
use crate::common::types::Level;

pub struct LimitOrderBook<const SIZE: usize, const IS_BID: bool> {
    price_map: PriceMap,
    price_top: PriceTop<SIZE, IS_BID>,
}

impl<const SIZE: usize, const IS_BID: bool> LimitOrderBook<SIZE, IS_BID> {
    pub fn new(start_px: f64, tick_size: f64) -> Self {
        LimitOrderBook {
            price_map: PriceMap::new(start_px, tick_size),
            price_top: PriceTop::new(tick_size),
        }
    }

    #[inline(always)]
    pub fn apply_updates<const IS_SNAPSHOT: bool>(&mut self, updates: &Vec<Level>) -> usize {
        if unlikely(IS_SNAPSHOT) {
            self.price_map.clear();
            self.price_top.clear();
        }

        for Level { px, amt } in updates.iter() {
            // Note:
            // We might skip the top update part by checking the condition (was_nonzero_amt || amt == 0.0),
            // but that would require reading the old amount from memory, which we want to avoid.
            self.price_map.get_mut(*px).amt = *amt;

            self.price_top.update(*px, *amt, |worst_px| {
                self.price_map.next_px::<IS_BID>(worst_px)
            });
        }

        updates.len()
    }

    pub fn top_levels_from_map<const N: usize>(&self) -> [Option<(f64, PriceLevel)>; N] {
        self.price_map.top_levels::<N, IS_BID>()
    }

    #[inline(always)]
    pub fn top_levels(&self) -> &Vec<f64> {
        self.price_top.top()
    }
}
