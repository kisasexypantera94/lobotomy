extern crate lobotomy;

use lobotomy::binance::*;
use lobotomy::common::communication::EventMessage;
use lobotomy::common::WebSocketListener;

use std::sync::mpsc;

fn main() {
    fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Info)
            .chan_len(Some(100000)),
    )
    .unwrap();

    const URL: &str = "wss://stream.binance.com:9443/ws/btcusdt@depth@100ms";

    let (counter_frequency, accuracy) = tick_counter::frequency();
    let frequency_base = match accuracy {
        tick_counter::TickCounterFrequencyBase::Hardware => "hardware provided".to_string(),
        tick_counter::TickCounterFrequencyBase::Measured(duration) => {
            format!("software estimated in {:?}", duration)
        }
    };
    println!(
        "Tick frequency, MHZ: {:.2} ({})",
        counter_frequency as f64 / 1e6_f64,
        frequency_base
    );

    let counter_accuracy = tick_counter::precision_nanoseconds(counter_frequency);
    println!("Tick accuracy, nanoseconds: {:.2}", counter_accuracy);

    let mut websocket_listener = WebSocketListener::new(URL);
    let (sender, receiver) = mpsc::channel::<EventMessage<MarketDataEvent>>();
    let depth_decoder = DepthDeltaDecoder::new();
    let mut restore_manager = RestoreManager::new();
    let mut limit_order_book = LimitOrderBook::<25>::new(34563.0, 0.01);

    // let mut core_ids = core_affinity::get_core_ids().unwrap().into_iter();

    std::thread::scope(|s| {
        // let core_id0 = core_ids.next().unwrap();
        // let core_id1 = core_ids.next().unwrap();

        let decoder_task = s.spawn(move || {
            // println!("Set core affinity: {}", core_affinity::set_for_current(core_id0));

            loop {
                let msg = receiver.try_recv();
                if msg.is_err() {
                    continue;
                }
                let msg = msg.unwrap();

                match msg {
                    EventMessage::Event(e) => {
                        let tick0 = tick_counter::start();
                        let num_updates = limit_order_book.apply_event(&e);
                        let tick1 = tick_counter::stop();

                        let (bids, asks) = limit_order_book.top_levels2();

                        log::info!(
                            "LOB: latency=[{}], latency_per_update=[{}], bids=[{:?}], asks=[{}]",
                            ((tick1 - tick0) as f64 * counter_accuracy).round() as usize / 1000,
                            ((tick1 - tick0) as f64 * counter_accuracy).round() as usize
                                / num_updates,
                            bids[0],
                            asks[0]
                        );

                        assert!(bids[0] < asks[0]);
                    }
                    EventMessage::Stop() => break,
                }
            }
        });

        let ws_task = s.spawn(move || {
            // println!(
            //     "Set core affinity: {}",
            //     core_affinity::set_for_current(core_id1)
            // );

            // let mut cnt = 0;

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
                // cnt += 1;
            }

            // let _ = sender.send(EventMessage::Stop());
        });

        std::thread::sleep(std::time::Duration::from_secs(10));

        decoder_task.join().unwrap();
        ws_task.join().unwrap();
    });
}
