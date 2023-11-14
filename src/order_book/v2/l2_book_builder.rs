use super::L2Book;
use crate::common::types::{Amount, L2Delta, Level, Price};

#[derive(Debug, Clone)]
pub struct L2BookBuilder<P: Price, A: Amount, const IS_BID: bool> {
    l2_book: L2Book<P, A, IS_BID>,
}

impl<P: Price, A: Amount, const IS_BID: bool> L2BookBuilder<P, A, IS_BID> {
    pub fn new(tick_size: P) -> Self {
        L2BookBuilder {
            l2_book: L2Book::new(tick_size),
        }
    }

    #[inline(always)]
    pub fn add_l2_upsert(&mut self, val: &Level<P, A>) -> usize {
        let lvl_idx = self.l2_book.insert(val.px);
        *self.l2_book.get_level_mut(lvl_idx) = val.amt;
        lvl_idx
    }

    #[inline(always)]
    pub fn add_l2_delta(&mut self, val: &L2Delta<P, A>) -> usize {
        let lvl_idx = self.l2_book.insert(val.px);

        let lvl = self.l2_book.get_level_mut(lvl_idx);
        *lvl = lvl.apply_delta(&val.amt_delta);
        lvl_idx
    }

    #[inline(always)]
    pub fn update_l2_upsert(&mut self, lvl_idx: usize, val: &Level<P, A>) {
        *self.l2_book.get_level_mut(lvl_idx) = val.amt;

        if val.amt.is_zero() {
            self.l2_book.delete(val.px);
        }
    }

    #[inline(always)]
    pub fn update_l2_delta(&mut self, lvl_idx: usize, val: &L2Delta<P, A>) {
        let lvl = self.l2_book.get_level_mut(lvl_idx);
        *lvl = lvl.apply_delta(&val.amt_delta);

        if lvl.is_zero() {
            self.l2_book.delete(val.px);
        }
    }

    #[inline(always)]
    pub fn book(&self) -> &L2Book<P, A, IS_BID> {
        &self.l2_book
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.l2_book.clear();
    }
}
