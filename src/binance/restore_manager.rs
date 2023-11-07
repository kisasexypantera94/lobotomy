use super::depth_diff_decoder::DepthDiff;
use crate::common::types::Level;

use serde::Deserialize;

use std::{error::Error, mem::swap};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct RawDepthSnapshot {
    lastUpdateId: u64,
    bids: Vec<(String, String)>,
    asks: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct DepthSnapshot {
    pub last_update_id: u64,
    pub bids: Vec<Level<f64, f64>>,
    pub asks: Vec<Level<f64, f64>>,
}

#[derive(Debug)]
pub enum MarketData {
    Snapshot(DepthSnapshot),
    Diff(DepthDiff),
}

pub struct RestoreManager {
    snapshot_url: String,
    is_restored: bool,
    diff_buffer: Vec<DepthDiff>,
    snapshot: Option<DepthSnapshot>,
    last_u: u64,
}

impl RestoreManager {
    pub fn new(snapshot_url: &str) -> Self {
        RestoreManager {
            snapshot_url: snapshot_url.to_string(),
            is_restored: false,
            diff_buffer: Vec::new(),
            snapshot: None,
            last_u: 0,
        }
    }

    pub fn apply_diff<MdProcessor>(&mut self, diff: DepthDiff, md_processor: &mut MdProcessor)
    where
        MdProcessor: FnMut(MarketData),
    {
        if self.last_u != 0 && diff.first_update_id != self.last_u + 1 {
            log::warn!("Gap detected!");
            self.is_restored = false;
        }

        self.last_u = diff.last_update_id;

        if !self.is_restored {
            self.diff_buffer.push(diff);

            if let Some(pos) = self.try_restore() {
                let mut snapshot = None;
                swap(&mut snapshot, &mut self.snapshot);

                md_processor(MarketData::Snapshot(snapshot.unwrap()));

                for diff in self.diff_buffer.drain(..).skip(pos) {
                    md_processor(MarketData::Diff(diff));
                }
            }

            return;
        }

        md_processor(MarketData::Diff(diff));
    }

    fn try_restore(&mut self) -> Option<usize> {
        if self.snapshot.is_none()
            || self.snapshot.as_ref().unwrap().last_update_id < self.diff_buffer[0].first_update_id
        {
            self.snapshot = Some(self.get_snapshot().unwrap());
        }

        let snapshot_update_id = self.snapshot.as_ref().unwrap().last_update_id;

        match self.diff_buffer.iter().position(|diff| {
            diff.first_update_id <= snapshot_update_id + 1
                && diff.last_update_id >= snapshot_update_id + 1
        }) {
            Some(pos) => {
                self.is_restored = true;
                Some(pos)
            }
            None => None,
        }
    }

    fn get_snapshot(&self) -> Result<DepthSnapshot, Box<dyn Error>> {
        let res = reqwest::blocking::get(&self.snapshot_url).unwrap();
        let text = res.text().unwrap();

        let raw: RawDepthSnapshot = serde_json::from_str(&text).unwrap();

        let str_to_f64 = |v: &Vec<(String, String)>| {
            v.iter()
                .map(|(px_str, amt_str)| Level {
                    px: px_str.parse().unwrap(),
                    amt: amt_str.parse().unwrap(),
                })
                .collect()
        };

        Ok(DepthSnapshot {
            last_update_id: raw.lastUpdateId,
            bids: str_to_f64(&raw.bids),
            asks: str_to_f64(&raw.asks),
        })
    }
}
