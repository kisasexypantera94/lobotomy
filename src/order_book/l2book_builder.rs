use super::L2Book;
use super::PriceLevel;
use super::PriceMap;
use crate::common::types::Level;

pub struct L2BookBuilder<const SIZE: usize, const IS_BID: bool> {
    price_map: PriceMap,
    l2book: L2Book<SIZE, IS_BID>,
}

impl<const SIZE: usize, const IS_BID: bool> L2BookBuilder<SIZE, IS_BID> {
    pub fn new(start_px: f64, end_px: Option<f64>, tick_size: f64) -> Self {
        L2BookBuilder {
            price_map: PriceMap::new(start_px, end_px, tick_size),
            l2book: L2Book::new(tick_size),
        }
    }

    #[inline(always)]
    pub fn apply_snapshot(&mut self, snapshot: &[Level]) {
        self.price_map.clear();
        self.l2book.clear();

        self.apply_updates(snapshot);
    }

    #[inline(always)]
    pub fn apply_delta(&mut self, delta: &[Level]) {
        self.apply_updates(delta);
    }

    #[inline(always)]
    fn apply_updates(&mut self, updates: &[Level]) {
        for Level { px, amt } in updates.iter() {
            // Note:
            // We might skip the top update part by checking the condition (was_nonzero_amt || amt == 0.0),
            // but that would require reading the old amount from memory, which we want to avoid.
            self.price_map.get_mut(*px).amt = *amt;

            self.l2book.update(*px, *amt, |worst_px| {
                self.price_map.next_px::<IS_BID>(worst_px)
            });
        }
    }

    pub fn top_levels_from_map<const N: usize>(&self) -> [Option<(f64, PriceLevel)>; N] {
        self.price_map.top_levels::<N, IS_BID>()
    }

    #[inline(always)]
    pub fn book(&self) -> &L2Book<SIZE, IS_BID> {
        &self.l2book
    }
}
