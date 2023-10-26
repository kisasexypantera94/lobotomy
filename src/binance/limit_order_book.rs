use crate::price_map::{PriceLevel, PriceMap};
use crate::price_top::PriceTop;

use super::depth_delta_decoder::Level;
use super::restore_manager::MarketDataEvent;

pub struct LimitOrderBook<const SIZE: usize> {
    bid_price_map: PriceMap,
    ask_price_map: PriceMap,
    best_bids: PriceTop<SIZE, true>,
    best_asks: PriceTop<SIZE, false>,
}

impl<const SIZE: usize> LimitOrderBook<SIZE> {
    pub fn new(start_px: f64, tick_size: f64) -> Self {
        LimitOrderBook {
            bid_price_map: PriceMap::new(start_px, tick_size),
            ask_price_map: PriceMap::new(start_px, tick_size),
            best_bids: PriceTop::new(tick_size),
            best_asks: PriceTop::new(tick_size),
        }
    }

    pub fn apply_event(&mut self, event: &MarketDataEvent) -> usize {
        let mut num_updates = 0;

        match event {
            MarketDataEvent::Snapshot(snapshot) => {
                self.bid_price_map.clear();
                self.ask_price_map.clear();
                self.best_bids.clear();
                self.best_asks.clear();

                num_updates += self.apply_depths::<true>(&snapshot.bids);
                num_updates += self.apply_depths::<false>(&snapshot.asks);
            }
            MarketDataEvent::Delta(delta) => {
                num_updates += self.apply_depths::<true>(&delta.bids);
                num_updates += self.apply_depths::<false>(&delta.asks);
            }
        }

        num_updates
    }

    pub fn top_levels_from_map<const N: usize>(
        &self,
    ) -> (
        [Option<(f64, PriceLevel)>; N],
        [Option<(f64, PriceLevel)>; N],
    ) {
        (
            self.bid_price_map.top_levels::<N, true>(),
            self.ask_price_map.top_levels::<N, false>(),
        )
    }

    pub fn top_levels(&self) -> (&Vec<f64>, &Vec<f64>) {
        (self.best_bids.top(), self.best_asks.top())
    }

    fn apply_depths<const BID: bool>(&mut self, depths: &Vec<Level>) -> usize {
        for Level { px, amt } in depths.iter() {
            if BID {
                self.bid_price_map.get_mut(*px).amt = *amt;
                self.best_bids.update(*px, *amt, |worst_px| {
                    self.bid_price_map.next_worst_px::<true>(worst_px)
                });
            } else {
                self.ask_price_map.get_mut(*px).amt = *amt;
                self.best_asks.update(*px, *amt, |worst_px| {
                    self.ask_price_map.next_worst_px::<false>(worst_px)
                });
            };
        }

        depths.len()
    }
}
