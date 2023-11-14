use std::{collections::HashMap, hash::Hash};

use crate::{
    common::{
        intrinsics::*,
        types::{L2Delta, Level},
    },
    nasdaq::Price4Wrapper,
    order_book::v2::L2BookBuilder,
};

use itchy::{Body, Message, Price4, Side};

#[derive(Clone, Copy)]
struct Order {
    side: Side,
    price: Price4,
    shares: u32,
    lvl_idx: usize,
}

struct OrderPool {
    orders: Vec<Option<Order>>,
}

impl OrderPool {
    pub fn new() -> Self {
        OrderPool {
            orders: Vec::with_capacity(2_usize.pow(32)),
        }
    }

    #[inline(always)]
    pub fn insert(&mut self, reference: u64, order: Order) {
        let order_opt = self.get_mut(&reference);
        *order_opt = Some(order);
    }

    #[inline(always)]
    pub fn get_mut(&mut self, reference: &u64) -> &mut Option<Order> {
        let reference = *reference as usize;

        if unlikely(self.orders.len() <= reference as usize) {
            self.orders.resize(reference + 1, None);
            log::debug!("Resize triggered: new_len=[{}]", self.orders.len());
        }

        &mut self.orders[reference]
    }

    #[inline(always)]
    pub fn get_mut_unwrap(&mut self, reference: &u64) -> &mut Order {
        self.get_mut(reference).as_mut().unwrap()
    }

    #[inline(always)]
    pub fn get(&self, reference: &u64) -> &Option<Order> {
        let reference = *reference as usize;
        &self.orders[reference]
    }
}

#[derive(Clone)]
struct StockLOB {
    bid: L2BookBuilder<Price4Wrapper, u32, true>,
    ask: L2BookBuilder<Price4Wrapper, u32, false>,
}

pub struct ItchL2BookBuilder {
    orders: OrderPool,
    lobs: Vec<StockLOB>,
}

impl ItchL2BookBuilder {
    pub fn new(tick_size: Price4Wrapper) -> Self {
        ItchL2BookBuilder {
            orders: OrderPool::new(),
            lobs: vec![
                StockLOB {
                    bid: L2BookBuilder::new(tick_size),
                    ask: L2BookBuilder::new(tick_size)
                };
                u16::MAX.into()
            ],
        }
    }

    #[allow(unused_variables)]
    #[inline(always)]
    pub fn apply_message(&mut self, msg: &Message) {
        match &msg.body {
            Body::AddOrder(add_order) => {
                self.orders.insert(
                    add_order.reference,
                    Order {
                        side: add_order.side,
                        price: add_order.price,
                        shares: add_order.shares,
                        lvl_idx: usize::MAX,
                    },
                );

                let l2_delta = L2Delta {
                    px: Price4Wrapper(add_order.price),
                    amt_delta: (add_order.shares as i64),
                };

                self.orders.get_mut_unwrap(&add_order.reference).lvl_idx =
                    self.add_l2_delta(msg.stock_locate as usize, add_order.side, &l2_delta);
            }
            Body::OrderExecuted {
                reference,
                executed,
                match_number,
            } => {
                let order = match self.orders.get_mut(reference) {
                    Some(o) => o,
                    None => return,
                };

                order.shares -= executed;

                let l2_delta = L2Delta {
                    px: Price4Wrapper(order.price),
                    amt_delta: (-1 * *executed as i64),
                };

                let side = order.side;
                let lvl_idx = order.lvl_idx;

                self.update_l2_delta(msg.stock_locate as usize, side, lvl_idx, &l2_delta);
            }
            Body::OrderExecutedWithPrice {
                reference,
                executed,
                match_number,
                printable,
                price,
            } => {
                let order = match self.orders.get_mut(reference) {
                    Some(o) => o,
                    None => return,
                };

                order.shares -= executed;

                let l2_delta = L2Delta {
                    px: Price4Wrapper(order.price),
                    amt_delta: (-1 * *executed as i64),
                };

                let side = order.side;
                let lvl_idx = order.lvl_idx;

                self.update_l2_delta(msg.stock_locate as usize, side, lvl_idx, &l2_delta);
            }
            Body::OrderCancelled {
                reference,
                cancelled,
            } => {
                let order = match self.orders.get_mut(reference) {
                    Some(o) => o,
                    None => return,
                };

                order.shares -= cancelled;

                let l2_delta = L2Delta {
                    px: Price4Wrapper(order.price),
                    amt_delta: (-1 * *cancelled as i64),
                };

                let side = order.side;
                let lvl_idx = order.lvl_idx;

                self.update_l2_delta(msg.stock_locate as usize, side, lvl_idx, &l2_delta);
            }
            Body::DeleteOrder { reference } => {
                let order = match self.orders.get(reference) {
                    Some(o) => o,
                    None => return,
                };

                let l2_delta = L2Delta {
                    px: Price4Wrapper(order.price),
                    amt_delta: (-1 * order.shares as i64),
                };

                self.update_l2_delta(
                    msg.stock_locate as usize,
                    order.side,
                    order.lvl_idx,
                    &l2_delta,
                );
            }
            Body::ReplaceOrder(replace_order) => {
                let old_order = match self.orders.get(&replace_order.old_reference) {
                    Some(o) => *o,
                    None => return,
                };

                self.orders.insert(
                    replace_order.new_reference,
                    Order {
                        side: old_order.side,
                        price: replace_order.price,
                        shares: replace_order.shares,
                        lvl_idx: usize::MAX,
                    },
                );

                self.update_l2_delta(
                    msg.stock_locate as usize,
                    old_order.side,
                    old_order.lvl_idx,
                    &L2Delta {
                        px: Price4Wrapper(old_order.price),
                        amt_delta: (-1 * old_order.shares as i64),
                    },
                );

                self.orders
                    .get_mut_unwrap(&replace_order.new_reference)
                    .lvl_idx = self.add_l2_delta(
                    msg.stock_locate as usize,
                    old_order.side,
                    &L2Delta {
                        px: Price4Wrapper(replace_order.price),
                        amt_delta: (replace_order.shares as i64),
                    },
                );
            }
            _ => (),
        }
    }

    #[inline(always)]
    fn add_l2_delta(
        &mut self,
        stock_locate: usize,
        side: Side,
        l2_delta: &L2Delta<Price4Wrapper, u32>,
    ) -> usize {
        match side {
            Side::Buy => self.lobs[stock_locate].bid.add_l2_delta(&l2_delta),
            Side::Sell => self.lobs[stock_locate].ask.add_l2_delta(&l2_delta),
        }
    }

    #[inline(always)]
    fn update_l2_delta(
        &mut self,
        stock_locate: usize,
        side: Side,
        lvl_idx: usize,
        l2_delta: &L2Delta<Price4Wrapper, u32>,
    ) {
        match side {
            Side::Buy => self.lobs[stock_locate]
                .bid
                .update_l2_delta(lvl_idx, &l2_delta),
            Side::Sell => self.lobs[stock_locate]
                .ask
                .update_l2_delta(lvl_idx, &l2_delta),
        };
    }
}
