extern crate lobotomy;

use lobotomy::common::communication::EventMessage;
use lobotomy::common::types::L2Delta;
use lobotomy::common::StackInvocable;
use lobotomy::nasdaq::{ItchIntoL2Deltas, Price4Wrapper};
use lobotomy::order_book::v1::L2BookBuilder;

use itchy::Body;
use more_asserts::assert_lt;
use rtrb::{Consumer, Producer, PushError, RingBuffer};

pub type Invocable = StackInvocable<32>;

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

fn async_task(mut async_consuemr: Consumer<EventMessage<Invocable>>) {
    loop {
        let msg = match async_consuemr.pop() {
            Ok(msg) => msg,
            Err(_) => {
                continue;
            }
        };

        match msg {
            EventMessage::Event(mut invocable) => invocable.invoke(),
            EventMessage::Stop => return,
        }
    }
}

fn limit_order_book_task(mut async_producer: Producer<EventMessage<Invocable>>) {
    const LOB_SIZE: usize = 2_usize.pow(14);

    #[derive(Clone)]
    struct StockLOB {
        bid: L2BookBuilder<Price4Wrapper, u32, LOB_SIZE, true>,
        ask: L2BookBuilder<Price4Wrapper, u32, LOB_SIZE, false>,
    }

    let counter_accuracy = calibrate_tick_counter();

    // https://emi.nasdaq.com/ITCH/Nasdaq%20ITCH/#:~:text=25%20AM%20%20%204075649457-,08302019.NASDAQ_ITCH50.gz,-8/31/2019
    let filename = "/Users/dvgr/dev/resources/08302019.NASDAQ_ITCH50";
    let stream = itchy::MessageStream::from_file(filename).unwrap();

    let stock_filter = [
        "GOOG", "NVDA", "AAPL", "TSLA", "AMZN", "AMD", "MSFT", "META", "GOOGL", "INTC", "AAL",
        "BKNG", "DKNG", "ROKU", "V", "UBER",
    ];

    let mut l2_from_itch = ItchIntoL2Deltas::new();
    let mut stock_to_lob = vec![None; 2_usize.pow(14)];

    for msg in stream {
        let msg = msg.unwrap();

        if let Body::StockDirectory(sd) = &msg.body {
            let stock = sd.stock.trim_end();
            if stock_filter.contains(&stock) {
                let start_px = Price4Wrapper(itchy::Price4::from(0));
                let end_px = None;
                let tick_size = Price4Wrapper(itchy::Price4::from(100));

                stock_to_lob.insert(
                    msg.stock_locate as usize,
                    Some(StockLOB {
                        bid: L2BookBuilder::new(start_px, end_px, tick_size),
                        ask: L2BookBuilder::new(start_px, end_px, tick_size),
                    }),
                )
            }
        };

        let mut had_updates = false;
        let tick0 = tick_counter::start();
        // ---------------------------------------------------------------------
        l2_from_itch.apply_message(&msg, |side, px, amt_delta| {
            if let Some(lob) = stock_to_lob[msg.stock_locate as usize].as_mut() {
                had_updates = true;
                match side {
                    itchy::Side::Buy => {
                        lob.bid.apply_l2_deltas(std::slice::from_ref(&L2Delta {
                            px: Price4Wrapper(*px),
                            amt_delta: *amt_delta,
                        }));
                    }
                    itchy::Side::Sell => {
                        lob.ask.apply_l2_deltas(std::slice::from_ref(&L2Delta {
                            px: Price4Wrapper(*px),
                            amt_delta: *amt_delta,
                        }));
                    }
                }

                // let b0 = match lob.bid.book().levels().get(0) {
                //     Some(b0) => *b0,
                //     None => Price4Wrapper::default(),
                // };

                // let a0 = match lob.ask.book().levels().get(0) {
                //     Some(a0) => *a0,
                //     None => Price4Wrapper::default(),
                // };

                // assert_lt!(b0, a0);

                // let async_task = Invocable::new(move || {
                //     println!(
                //         "time=[{}], stock=[{}], b0=[{}], a0=[{}]",
                //         msg.timestamp,
                //         msg.stock_locate,
                //         &f64::from(b0),
                //         &f64::from(a0),
                //     );
                // });

                // let mut item = EventMessage::Event(async_task);
                // while let Err(PushError::Full(i)) = async_producer.push(item) {
                //     // println!("MarketData queue is full!");
                //     item = i;
                //     continue;
                // }
            }
        });
        // ---------------------------------------------------------------------
        let tick1 = tick_counter::start();

        if !had_updates {
            continue;
        }

        let async_task = Invocable::new(move || {
            println!(
                "latency=[{}]",
                ((tick1 - tick0) as f64 * counter_accuracy).round() as usize,
            );
        });

        let mut item = EventMessage::Event(async_task);
        while let Err(PushError::Full(i)) = async_producer.push(item) {
            // println!("MarketData queue is full!");
            item = i;
            continue;
        }
    }

    while let Err(_) = async_producer.push(EventMessage::Stop) {}
}

fn main() {
    let (async_producer, async_consumer) = RingBuffer::new(2_usize.pow(32));

    std::thread::scope(move |s| {
        s.spawn(move || {
            async_task(async_consumer);
        });

        limit_order_book_task(async_producer);
    });
}
