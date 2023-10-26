extern crate lobotomy;

use lobotomy::binance::*;
use lobotomy::common::communication::EventMessage;
use lobotomy::common::WebSocketListener;
use lobotomy::order_book::LimitOrderBook;

use std::sync::mpsc;

fn init_log() {
    fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Info)
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

fn limit_order_book_task(receiver: &mpsc::Receiver<EventMessage<MarketDataEvent>>) {
    let counter_accuracy = calibrate_tick_counter();

    let start_px = 34176.0;
    let tick_size = 0.01;
    const LOB_SIZE: usize = 5;
    let mut bid_lob = LimitOrderBook::<LOB_SIZE, true>::new(start_px, tick_size);
    let mut ask_lob = LimitOrderBook::<LOB_SIZE, false>::new(start_px, tick_size);

    loop {
        let msg = match receiver.try_recv() {
            Ok(msg) => msg,
            Err(_) => {
                continue;
            }
        };

        match &msg {
            EventMessage::Event(e) => {
                let tick0 = tick_counter::start();
                let num_updates = match &e {
                    MarketDataEvent::Delta(delta) => {
                        bid_lob.apply_updates::<false>(&delta.bids)
                            + ask_lob.apply_updates::<false>(&delta.asks)
                    }
                    MarketDataEvent::Snapshot(snapshot) => {
                        bid_lob.apply_updates::<true>(&snapshot.bids)
                            + ask_lob.apply_updates::<true>(&snapshot.asks)
                    }
                };
                let tick1 = tick_counter::stop();

                let bids = bid_lob.top_levels();
                let asks = ask_lob.top_levels();

                log::info!(
                    "LOB: latency=[{}], latency_per_update=[{}], bids=[{:?}], asks=[{}]",
                    ((tick1 - tick0) as f64 * counter_accuracy).round() as usize / 1000,
                    ((tick1 - tick0) as f64 * counter_accuracy).round() as usize / num_updates,
                    bids[0],
                    asks[0]
                );

                assert!(bids[0] < asks[0]);
            }
            EventMessage::Stop() => break,
        }
    }
}

fn marketdata_task(sender: &mpsc::Sender<EventMessage<MarketDataEvent>>) {
    const URL: &str = "wss://stream.binance.com:9443/ws/btcusdt@depth@100ms";

    let mut websocket_listener = WebSocketListener::new(URL);
    let depth_decoder = DepthDeltaDecoder::new();
    let mut restore_manager = RestoreManager::new();

    loop {
        let msg = websocket_listener.read().unwrap();
        let depth_delta = match depth_decoder.decode(&msg) {
            Ok(ev) => ev,
            Err(_) => {
                log::error!("Could not decode depth event: msg=[{}]", msg);
                continue;
            }
        };

        restore_manager.apply_depth(depth_delta, &mut |md_event| {
            let _ = sender.send(EventMessage::Event(md_event));
        });
    }
}

fn main() {
    init_log();

    let (sender, receiver) = mpsc::channel::<EventMessage<MarketDataEvent>>();

    std::thread::scope(|s| {
        s.spawn(move || {
            limit_order_book_task(&receiver);
        });

        s.spawn(move || {
            marketdata_task(&sender);
        });
    });
}
