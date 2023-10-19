pub fn px_to_tick_idx(px: f64, tick_size: f64) -> usize {
    (px / tick_size).round() as usize
}

pub fn round_to_tick_size(val: f64, tick_size: f64) -> f64 {
    let mult = 1.0 / tick_size;
    (val * mult + 0.5).floor() / mult
}
