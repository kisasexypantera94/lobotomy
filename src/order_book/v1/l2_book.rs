use crate::common::intrinsics::*;
use crate::common::types::{Amount, Level, Price};

#[derive(Debug, Clone)]
pub struct L2Book<P, const N: usize, const REVERSE: bool> {
    levels: Vec<P>,
    tick_size: P,
}

/// Cases:
///
/// - Amount > 0 (Upsert):
///     1. Find the position for insertion/update. If the position is beyond the top (None), then take no action.
///     2. If we find the exact price, simply return.
///     3. Shift to the right from the insertion position.
///     4. Insert the new price.
///
/// - Amount == 0 (Delete):
///     1. Find the position for the price. If the position is beyond the top (None), take no action.
///        Also, find the position of the worst price in the top.
///     2. If `top[pos] != price`, return.
///     3. If the position of the worst price is None, it means the top is empty - return.
///     4. When we delete the price, there will be a shift to the left,
///        leaving an empty spot at the position of the worst price. Therefore, we need to ask PriceMap for the next worst price with amount > 0.
///     5. Insert the next worst price in the empty spot.
impl<P: Price, const N: usize, const REVERSE: bool> L2Book<P, N, REVERSE> {
    pub fn new(tick_size: P) -> Self {
        L2Book {
            levels: Vec::with_capacity(N),
            tick_size,
        }
    }

    #[inline(always)]
    pub fn upsert(&mut self, px: P) {
        let px = P::round_to_tick_size(&px, &self.tick_size);
        let mut px_pos_opt = self.levels.is_empty().then_some(0);

        for (idx, lvl) in self.levels.iter().enumerate() {
            if Self::comparator(px, *lvl) {
                px_pos_opt = Some(idx);
                break;
            }
        }

        let px_pos = match px_pos_opt {
            Some(pos) => pos,
            None => {
                self.levels.push(px);
                return;
            }
        };

        if unlikely(self.levels.is_empty()) {
            self.levels.push(px);
            return;
        }

        if self.levels[px_pos] == px {
            return;
        }

        if likely(self.levels.len() == N) {
            self.levels.pop();
        }

        self.levels.insert(px_pos, px);
    }

    #[inline(always)]
    pub fn delete<A: Amount>(
        &mut self,
        px: P,
        get_next_worst_lvl: impl Fn(&P) -> Option<Level<P, A>>,
    ) {
        if unlikely(self.levels.is_empty()) {
            return;
        }

        let px = P::round_to_tick_size(&px, &self.tick_size);
        let mut px_pos_opt = None;
        let worst_pos = self.levels.len() - 1;

        for (idx, lvl) in self.levels.iter().enumerate() {
            if px == *lvl {
                px_pos_opt = Some(idx);
                break;
            }
        }

        let px_pos = match px_pos_opt {
            Some(pos) => pos,
            None => return,
        };

        let worst_lvl = self.levels[worst_pos];

        self.levels.remove(px_pos);

        if self.levels.len() < N - 1 {
            return;
        }

        let lvl = match get_next_worst_lvl(&worst_lvl) {
            Some(lvl) => lvl,
            None => return,
        };

        self.levels
            .push(P::round_to_tick_size(&lvl.px, &self.tick_size));
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
    pub fn levels(&self) -> &[P] {
        &self.levels
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.levels.clear();
    }
}
