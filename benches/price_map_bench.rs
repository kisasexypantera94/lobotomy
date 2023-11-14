#![feature(test)]

extern crate lobotomy;
extern crate test;

use lobotomy::order_book::v1::PriceMap;
use rand::Rng;

const TICK_SIZE: f64 = 0.1;
const PX_RANGE: std::ops::Range<f64> = 128.0..256.0;

fn round_to_tick(val: f64) -> f64 {
    let mult = 1.0 / TICK_SIZE;
    (val * mult + 0.5).floor() / mult
}

fn prepare_history() -> Vec<(f64, f64)> {
    let history_len = 100_000_000;
    let amt_range = 0.0..1_000_000.0;

    let mut history = Vec::with_capacity(history_len);

    for _ in 0..history_len {
        history.push((
            round_to_tick(rand::thread_rng().gen_range(PX_RANGE)),
            rand::thread_rng().gen_range(amt_range.clone()),
        ));
    }

    history
}

#[bench]
fn price_map_bench(b: &mut test::Bencher) {
    let history = prepare_history();
    let mut iter = history.iter().cycle();

    let mut fast_map = PriceMap::new(
        round_to_tick(rand::thread_rng().gen_range(PX_RANGE)),
        None,
        TICK_SIZE,
    );

    b.iter(std::hint::black_box(|| {
        let (px, amt) = iter.next().unwrap();
        fast_map.get_mut(*px).amt = *amt;
    }));
}

#[bench]
fn naive_map_bench(b: &mut test::Bencher) {
    let history = prepare_history();
    let mut iter = history.iter().cycle();

    let mut naive_map = std::collections::HashMap::<usize, f64>::new();

    b.iter(std::hint::black_box(|| {
        let (px, amt) = iter.next().unwrap();
        naive_map.insert((*px / TICK_SIZE).round() as usize, *amt);
    }));
}
