use crate::common::types::Level;

use itchy::{ArrayString8, Body, Price4, Side};

use std::collections::{HashMap, HashSet};

struct Order {
    stock: ArrayString8,
    side: Side,
    price: Price4,
    shares: u32,
}

#[inline(always)]
fn price4_into_f64(price: &Price4) -> f64 {
    const PRICE4_SCALE: f64 = 0.0001;
    price.raw() as f64 * PRICE4_SCALE
}

pub struct ItchIntoL2Deltas {
    orders: HashMap<u64, Order>,
    stock_filter: HashSet<ArrayString8>,
}

impl ItchIntoL2Deltas {
    pub fn new(symbol_filter: &[String]) -> Self {
        ItchIntoL2Deltas {
            orders: HashMap::new(),
            stock_filter: symbol_filter
                .iter()
                .map(|s| ArrayString8::from(s).unwrap())
                .collect(),
        }
    }

    #[allow(unused_variables)]
    #[inline(always)]
    pub fn apply_message(
        &mut self,
        body: &Body,
        mut process_l2_delta: impl FnMut(&ArrayString8, &Side, &Level),
    ) {
        match body {
            Body::AddOrder(add_order) => {
                let stock = ArrayString8::from(add_order.stock.trim_end()).unwrap();
                if !self.stock_filter.contains(&stock) {
                    return;
                }

                self.orders.insert(
                    add_order.reference,
                    Order {
                        stock,
                        side: add_order.side,
                        price: add_order.price,
                        shares: add_order.shares,
                    },
                );
                process_l2_delta(
                    &stock,
                    &add_order.side,
                    &Level {
                        px: price4_into_f64(&add_order.price),
                        amt: add_order.shares as f64,
                    },
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

                let stock = order.stock;
                let side = order.side;
                let l2_delta = Level {
                    px: price4_into_f64(&order.price),
                    amt: -1.0 * *executed as f64,
                };

                if order.shares == 0 {
                    self.orders.remove(reference);
                }

                process_l2_delta(&stock, &side, &l2_delta);
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

                let stock = order.stock;
                let side = order.side;
                let l2_delta = Level {
                    px: price4_into_f64(&order.price),
                    amt: -1.0 * *executed as f64,
                };

                if order.shares == 0 {
                    self.orders.remove(reference);
                }

                process_l2_delta(&stock, &side, &l2_delta);
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

                let stock = order.stock;
                let side = order.side;
                let l2_delta = Level {
                    px: price4_into_f64(&order.price),
                    amt: -1.0 * *cancelled as f64,
                };

                if order.shares == 0 {
                    self.orders.remove(reference);
                }

                process_l2_delta(&stock, &side, &l2_delta);
            }
            Body::DeleteOrder { reference } => {
                let order = match self.orders.remove(reference) {
                    Some(o) => o,
                    None => return,
                };

                process_l2_delta(
                    &order.stock,
                    &order.side,
                    &Level {
                        px: price4_into_f64(&order.price),
                        amt: -1.0 * order.shares as f64,
                    },
                );
            }
            Body::ReplaceOrder(replace_order) => {
                let old_order = match self.orders.remove(&replace_order.old_reference) {
                    Some(o) => o,
                    None => return,
                };

                self.orders.insert(
                    replace_order.new_reference,
                    Order {
                        stock: old_order.stock,
                        side: old_order.side,
                        price: replace_order.price,
                        shares: replace_order.shares,
                    },
                );

                process_l2_delta(
                    &old_order.stock,
                    &old_order.side,
                    &Level {
                        px: price4_into_f64(&old_order.price),
                        amt: -1.0 * old_order.shares as f64,
                    },
                );

                process_l2_delta(
                    &old_order.stock,
                    &old_order.side,
                    &Level {
                        px: price4_into_f64(&replace_order.price),
                        amt: replace_order.shares as f64,
                    },
                );
            }
            _ => (),
        }
    }

    pub fn len(&self) -> usize {
        self.orders.len()
    }
}
