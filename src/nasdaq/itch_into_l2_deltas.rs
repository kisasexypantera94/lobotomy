use crate::common::intrinsics::*;

use itchy::{Body, Message, Price4, Side};

#[derive(Clone, Copy)]
struct Order {
    side: Side,
    price: Price4,
    shares: u32,
}

struct OrderPool {
    orders: Vec<Option<Order>>,
}

impl OrderPool {
    pub fn new() -> Self {
        OrderPool {
            orders: Vec::with_capacity(2_usize.pow(30)),
        }
    }

    #[inline(always)]
    pub fn insert(&mut self, reference: u64, order: Order) {
        *self.get_mut(&reference) = Some(order);
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
    pub fn get(&self, reference: &u64) -> &Option<Order> {
        let reference = *reference as usize;
        &self.orders[reference]
    }
}

pub struct ItchIntoL2Deltas {
    orders: OrderPool,
}

impl ItchIntoL2Deltas {
    pub fn new() -> Self {
        ItchIntoL2Deltas {
            orders: OrderPool::new(),
        }
    }

    #[allow(unused_variables)]
    #[inline(always)]
    pub fn apply_message(
        &mut self,
        msg: &Message,
        mut process_l2_delta: impl FnMut(&Side, &Price4, &i64),
    ) {
        match &msg.body {
            Body::AddOrder(add_order) => {
                self.orders.insert(
                    add_order.reference,
                    Order {
                        side: add_order.side,
                        price: add_order.price,
                        shares: add_order.shares,
                    },
                );

                process_l2_delta(
                    &add_order.side,
                    &add_order.price,
                    &(add_order.shares as i64),
                );
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

                process_l2_delta(&order.side, &order.price, &(-1 * *executed as i64));
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

                process_l2_delta(&order.side, &order.price, &(-1 * *executed as i64));
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

                process_l2_delta(&order.side, &order.price, &(-1 * *cancelled as i64));
            }
            Body::DeleteOrder { reference } => {
                let order = match self.orders.get(reference) {
                    Some(o) => o,
                    None => return,
                };

                process_l2_delta(&order.side, &order.price, &(-1 * order.shares as i64));
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
                    },
                );

                process_l2_delta(
                    &old_order.side,
                    &old_order.price,
                    &(-1 * old_order.shares as i64),
                );

                process_l2_delta(
                    &old_order.side,
                    &replace_order.price,
                    &(replace_order.shares as i64),
                );
            }
            _ => (),
        }
    }
}
