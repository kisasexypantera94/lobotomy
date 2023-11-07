extern crate lobotomy;

use lobotomy::binance::{DepthDiffDecoder, MarketData, RestoreManager};
use lobotomy::common::communication::EventMessage;
use lobotomy::common::WebSocketListener;
use lobotomy::order_book::L2BookBuilder;

use heapless::spsc; // std::sync::mpsc was causing a segfault

const QUEUE_SIZE: usize = 16;

fn init_log() {
    fast_log::init(
        fast_log::Config::new()
            .console()
            .file("binance_robot.log")
            .level(log::LevelFilter::Debug)
            .chan_len(Some(100000)),
    )
    .unwrap();
}

fn calibrate_tick_counter() -> f64 {
    let (counter_frequency, accuracy) = tick_counter::frequency();
    let frequency_base = match accuracy {
        tick_counter::TickCounterFrequencyBase::Hardware => "hardware provided".to_string(),
        tick_counter::TickCounterFrequencyBase::Measured(duration) => {
            format!("software estimated in {:?}", duration)
        }
    };
    log::info!(
        "Tick frequency, MHZ: {:.2} ({})",
        counter_frequency as f64 / 1e6_f64,
        frequency_base
    );

    let counter_accuracy = tick_counter::precision_nanoseconds(counter_frequency);
    log::info!("Tick accuracy, nanoseconds: {:.2}", counter_accuracy);

    counter_accuracy
}

fn limit_order_book_task(mut md_receiver: spsc::Consumer<EventMessage<MarketData>, QUEUE_SIZE>) {
    let counter_accuracy = calibrate_tick_counter();

    let start_px = 0.0;
    let end_px = None;
    let tick_size = 0.01;
    const LOB_SIZE: usize = 2_usize.pow(14);
    let mut bid_lob_builder =
        L2BookBuilder::<f64, f64, LOB_SIZE, true>::new(start_px, end_px, tick_size);
    let mut ask_lob_builder =
        L2BookBuilder::<f64, f64, LOB_SIZE, false>::new(start_px, end_px, tick_size);

    loop {
        let msg = match md_receiver.dequeue() {
            Some(msg) => msg,
            None => {
                continue;
            }
        };

        match &msg {
            EventMessage::Event(md) => {
                let tick0 = tick_counter::start();
                let num_updates = match &md {
                    MarketData::Diff(diff) => {
                        bid_lob_builder.apply_l2_upserts(&diff.bids);
                        ask_lob_builder.apply_l2_upserts(&diff.asks);

                        diff.bids.len() + diff.asks.len()
                    }
                    MarketData::Snapshot(snapshot) => {
                        bid_lob_builder.apply_l2_snapshot(&snapshot.bids);
                        ask_lob_builder.apply_l2_snapshot(&snapshot.asks);

                        snapshot.bids.len() + snapshot.asks.len()
                    }
                };
                let tick1 = tick_counter::stop();

                let bids = bid_lob_builder.book().levels();
                let asks = ask_lob_builder.book().levels();

                log::info!(
                    "latency=[{}], bids=[{}], asks=[{}]",
                    ((tick1 - tick0) as f64 * counter_accuracy).round() as usize / num_updates,
                    bids[0],
                    asks[0]
                );

                assert!(bids[0] < asks[0]);
            }
            EventMessage::Stop => break,
        }
    }
}

fn marketdata_task(mut md_sender: spsc::Producer<EventMessage<MarketData>, QUEUE_SIZE>) {
    const DIFF_URL: &str = "wss://stream.binance.com:9443/ws/btcusdt@depth@100ms";
    const SNAPSHOT_URL: &str = "https://api.binance.com/api/v3/depth?symbol=BTCUSDT&limit=5000";

    let mut websocket_listener = WebSocketListener::new(DIFF_URL);
    let depth_diff_decoder = DepthDiffDecoder::new();
    let mut restore_manager = RestoreManager::new(SNAPSHOT_URL);

    loop {
        let msg = websocket_listener.read().unwrap();
        let depth_diff = match depth_diff_decoder.decode(&msg) {
            Ok(diff) => diff,
            Err(_) => {
                log::error!("Could not decode depth event: msg=[{}]", msg);
                continue;
            }
        };

        restore_manager.apply_diff(depth_diff, &mut |md_event| {
            let mut item = EventMessage::Event(md_event);

            while let Err(i) = md_sender.enqueue(item) {
                log::warn!("MarketData queue is full!");
                item = i;
                continue;
            }
        });
    }
}

fn main() {
    init_log();

    let mut md_queue = spsc::Queue::<EventMessage<MarketData>, QUEUE_SIZE>::new();
    let (md_sender, md_receiver) = md_queue.split();

    std::thread::scope(|s| {
        s.spawn(move || {
            limit_order_book_task(md_receiver);
        });

        s.spawn(move || {
            marketdata_task(md_sender);
        });
    });
}
