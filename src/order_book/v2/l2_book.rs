use crate::common::types::{Amount, Price};
use crate::common::{intrinsics::*, ObjectPool};

#[derive(Debug, Clone, Copy)]
pub struct BookEntry<P> {
    pub px: P,
    lvl_idx: usize,
}

#[derive(Debug, Clone)]
pub struct L2Book<P, A: Amount, const REVERSE: bool> {
    levels: ObjectPool<A>,
    book: Vec<BookEntry<P>>,
    tick_size: P,
}

impl<P: Price, A: Amount, const REVERSE: bool> L2Book<P, A, REVERSE> {
    pub fn new(tick_size: P) -> Self {
        L2Book {
            levels: ObjectPool::new(2_usize.pow(14)),
            book: Vec::with_capacity(2_usize.pow(14)),
            tick_size,
        }
    }

    #[inline(always)]
    pub fn insert(&mut self, px: P) -> usize {
        let px = P::round_to_tick_size(&px, &self.tick_size);

        let px_pos_opt = self
            .book
            .iter()
            .position(|lvl| Self::comparator(px, lvl.px));

        let px_pos = match px_pos_opt {
            Some(pos) => pos,
            None => {
                let lvl_idx = self.levels.allocate();
                self.book.push(BookEntry { px, lvl_idx });
                return lvl_idx;
            }
        };

        if self.book[px_pos].px == px {
            return self.book[px_pos].lvl_idx;
        }

        let lvl_idx = self.levels.allocate();
        self.book.insert(px_pos, BookEntry { px, lvl_idx });
        lvl_idx
    }

    #[inline(always)]
    pub fn get_level_mut(&mut self, lvl_idx: usize) -> &mut A {
        self.levels.get_mut(lvl_idx)
    }

    #[inline(always)]
    pub fn delete(&mut self, px: P) {
        if unlikely(self.book.is_empty()) {
            return;
        }

        let px = P::round_to_tick_size(&px, &self.tick_size);

        let px_pos = match self.book.iter().position(|lvl| px == lvl.px) {
            Some(pos) => pos,
            None => return,
        };

        let entry = self.book.remove(px_pos);
        self.levels.free(entry.lvl_idx);
    }

    #[inline(always)]
    fn comparator(a: P, b: P) -> bool {
        if REVERSE {
            a >= b
        } else {
            a <= b
        }
    }

    #[inline(always)]
    pub fn levels(&self) -> &[BookEntry<P>] {
        &self.book
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.book.clear();
    }
}
