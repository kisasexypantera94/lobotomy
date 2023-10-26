use crate::common::types::Level;

use serde::Deserialize;

use std::error::Error;

#[allow(non_snake_case, dead_code)]
#[derive(Debug, Deserialize)]
struct RawDepthDeltaEvent {
    e: String,
    E: u64,
    s: String,
    U: u64,
    u: u64,
    b: Vec<(String, String)>,
    a: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct DepthDeltaEvent {
    pub timestamp: u64,
    pub symbol: String,
    pub first_update_id: u64,
    pub last_update_id: u64,
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
}

pub struct DepthDeltaDecoder {}

impl DepthDeltaDecoder {
    pub fn new() -> Self {
        DepthDeltaDecoder {}
    }

    pub fn decode(&self, text: &str) -> Result<DepthDeltaEvent, Box<dyn Error>> {
        let raw: RawDepthDeltaEvent = serde_json::from_str(text)?;

        let str_to_f64 = |v: &Vec<(String, String)>| {
            v.iter()
                .map(|(px_str, amt_str)| Level {
                    px: px_str.parse().unwrap(),
                    amt: amt_str.parse().unwrap(),
                })
                .collect()
        };

        Ok(DepthDeltaEvent {
            timestamp: raw.E,
            symbol: raw.s,
            first_update_id: raw.U,
            last_update_id: raw.u,
            bids: str_to_f64(&raw.b),
            asks: str_to_f64(&raw.a),
        })
    }
}
