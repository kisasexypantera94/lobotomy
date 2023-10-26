extern crate lobotomy;

use lobotomy::order_book::PriceMap;
use rand::Rng;

use std::collections::HashMap;

#[test]
fn price_map_test() {
    let history_len = 10_000_000;
    let tick_size = 0.001;
    let px_range = 128.0..256.0;
    let amt_range = 0.0..1_000_000.0;

    let round_to_tick = |val: f64| {
        let mult = 1.0 / tick_size;
        (val * mult + 0.5).floor() / mult
    };

    let mut history: Vec<(f64, f64)> = Vec::with_capacity(history_len);

    for _ in 0..history_len {
        history.push((
            round_to_tick(rand::thread_rng().gen_range(px_range.clone())),
            rand::thread_rng().gen_range(amt_range.clone()),
        ));
    }

    let mut fast_map = PriceMap::new(
        round_to_tick(rand::thread_rng().gen_range(px_range)),
        tick_size,
    );
    let mut naive_map = HashMap::<usize, f64>::new();

    for (px, amt) in history.iter() {
        fast_map.get_mut(*px).amt = *amt;
        naive_map.insert((*px / tick_size).round() as usize, *amt);
    }

    for (tick_idx, amt) in naive_map.iter() {
        assert_eq!(fast_map.get_immut(*tick_idx as f64 * tick_size).amt, *amt);
    }
}
