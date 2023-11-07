use super::L2Book;
use super::PriceLevel;
use super::PriceMap;
use crate::common::types::{Amount, L2Delta, Level, Price};

#[derive(Debug, Clone)]
pub struct L2BookBuilder<P: Price, A: Amount, const SIZE: usize, const IS_BID: bool> {
    price_map: PriceMap<P, A>,
    l2_book: L2Book<P, SIZE, IS_BID>,
}

impl<P: Price, A: Amount, const SIZE: usize, const IS_BID: bool> L2BookBuilder<P, A, SIZE, IS_BID> {
    pub fn new(start_px: P, end_px: Option<P>, tick_size: P) -> Self {
        L2BookBuilder {
            price_map: PriceMap::new(start_px, end_px, tick_size),
            l2_book: L2Book::new(tick_size),
        }
    }

    #[inline(always)]
    pub fn apply_l2_snapshot(&mut self, l2_snapshot: &[Level<P, A>]) {
        self.price_map.clear();
        self.l2_book.clear();

        self.apply_l2_upserts(l2_snapshot);
    }

    #[inline(always)]
    pub fn apply_l2_upserts(&mut self, l2_updates: &[Level<P, A>]) {
        for Level { px, amt } in l2_updates.iter() {
            let level = self.price_map.get_mut(*px);

            let was_zero = level.amt.is_zero() && !amt.is_zero();
            level.amt = *amt;
            let became_zero = !was_zero && level.amt.is_zero();

            if was_zero {
                self.l2_book.upsert(*px);
            } else if became_zero {
                self.l2_book
                    .delete(*px, |worst_px| self.price_map.next_px::<IS_BID>(worst_px));
            }
        }
    }

    #[inline(always)]
    pub fn apply_l2_deltas(&mut self, l2_deltas: &[L2Delta<P, A>]) {
        for L2Delta { px, amt_delta } in l2_deltas.iter() {
            let level = self.price_map.get_mut(*px);

            let was_zero = level.amt.is_zero();
            level.amt = level.amt.apply_delta(amt_delta);
            let became_zero = level.amt.is_zero();

            if was_zero {
                self.l2_book.upsert(*px);
            } else if became_zero {
                self.l2_book
                    .delete(*px, |worst_px| self.price_map.next_px::<IS_BID>(worst_px));
            }
        }
    }

    pub fn top_levels_from_map<const N: usize>(&self) -> [Option<(P, PriceLevel<A>)>; N] {
        self.price_map.top_levels::<N, IS_BID>()
    }

    #[inline(always)]
    pub fn book(&self) -> &L2Book<P, SIZE, IS_BID> {
        &self.l2_book
    }

    #[inline(always)]
    pub fn get_level(&self, px: P) -> PriceLevel<A> {
        self.price_map.get_immut(px)
    }
}
