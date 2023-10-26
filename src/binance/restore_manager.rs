use super::depth_delta_decoder::DepthDeltaEvent;
use crate::common::types::Level;

use serde::Deserialize;

use std::{error::Error, mem::swap};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct RawDepthSnapshotEvent {
    lastUpdateId: u64,
    bids: Vec<(String, String)>,
    asks: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct DepthSnapshotEvent {
    pub last_update_id: u64,
    pub bids: Vec<Level>,
    pub asks: Vec<Level>,
}

#[derive(Debug)]
pub enum MarketDataEvent {
    Snapshot(DepthSnapshotEvent),
    Delta(DepthDeltaEvent),
}

pub struct RestoreManager {
    is_restored: bool,
    events_buffer: Vec<DepthDeltaEvent>,
    snapshot: Option<DepthSnapshotEvent>,
    last_u: u64,
}

impl RestoreManager {
    pub fn new() -> Self {
        RestoreManager {
            is_restored: false,
            events_buffer: Vec::new(),
            snapshot: None,
            last_u: 0,
        }
    }

    pub fn apply_depth<MdEventProcessor>(
        &mut self,
        depth_events: DepthDeltaEvent,
        md_event_processor: &mut MdEventProcessor,
    ) where
        MdEventProcessor: FnMut(MarketDataEvent),
    {
        if self.last_u != 0 && depth_events.first_update_id != self.last_u + 1 {
            log::warn!("Gap detected!");
            self.is_restored = false;
        }

        self.last_u = depth_events.last_update_id;

        if !self.is_restored {
            self.events_buffer.push(depth_events);

            if let Some(pos) = self.try_restore() {
                let mut snapshot = None;
                swap(&mut snapshot, &mut self.snapshot);

                md_event_processor(MarketDataEvent::Snapshot(snapshot.unwrap()));

                for depth_event in self.events_buffer.drain(..).skip(pos) {
                    md_event_processor(MarketDataEvent::Delta(depth_event));
                }
            }

            return;
        }

        md_event_processor(MarketDataEvent::Delta(depth_events));
    }

    fn try_restore(&mut self) -> Option<usize> {
        if self.snapshot.is_none()
            || self.snapshot.as_ref().unwrap().last_update_id
                < self.events_buffer[0].first_update_id
        {
            self.snapshot = Some(self.get_snapshot().unwrap());
        }

        let snapshot_update_id = self.snapshot.as_ref().unwrap().last_update_id;

        match self.events_buffer.iter().position(|events| {
            events.first_update_id <= snapshot_update_id + 1
                && events.last_update_id >= snapshot_update_id + 1
        }) {
            Some(pos) => {
                self.is_restored = true;
                Some(pos)
            }
            None => None,
        }
    }

    fn get_snapshot(&self) -> Result<DepthSnapshotEvent, Box<dyn Error>> {
        const URL: &str = "https://api.binance.com/api/v3/depth?symbol=BTCUSDT&limit=5000";

        let res = reqwest::blocking::get(URL).unwrap();
        let text = res.text().unwrap();

        let raw: RawDepthSnapshotEvent = serde_json::from_str(&text).unwrap();

        let str_to_f64 = |v: &Vec<(String, String)>| {
            v.iter()
                .map(|(px_str, amt_str)| Level {
                    px: px_str.parse().unwrap(),
                    amt: amt_str.parse().unwrap(),
                })
                .collect()
        };

        Ok(DepthSnapshotEvent {
            last_update_id: raw.lastUpdateId,
            bids: str_to_f64(&raw.bids),
            asks: str_to_f64(&raw.asks),
        })
    }
}
