extern crate lobotomy;

use std::collections::HashMap;

use lobotomy::common::communication::EventMessage;
use lobotomy::common::StackInvocable;
use lobotomy::nasdaq::ItchIntoL2Deltas;
use lobotomy::order_book::L2BookBuilder;

use itchy::ArrayString8;
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

fn async_task(mut consumer: Consumer<EventMessage<Invocable>>) {
    loop {
        let msg = match consumer.pop() {
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

fn limit_order_book_task(mut producer: Producer<EventMessage<Invocable>>) {
    const LOB_SIZE: usize = 25;

    struct StockLOB {
        bid: L2BookBuilder<LOB_SIZE, true>,
        ask: L2BookBuilder<LOB_SIZE, false>,
    }

    let counter_accuracy = calibrate_tick_counter();

    // https://emi.nasdaq.com/ITCH/Nasdaq%20ITCH/#:~:text=25%20AM%20%20%204075649457-,08302019.NASDAQ_ITCH50.gz,-8/31/2019
    let filename = "/Users/dvgr/dev/resources/08302019.NASDAQ_ITCH50";
    let stream = itchy::MessageStream::from_file(filename).unwrap();

    let stock_filter = [
        "GOOG", "NVDA", "AAAP", "TSLA", "AMZN", "AMD", "MSFT", "META", "GOOGL", "INTC",
    ]
    .map(String::from);
    let mut l2_from_itch = ItchIntoL2Deltas::new(&stock_filter);
    let mut stock_to_lob = HashMap::<ArrayString8, StockLOB>::new();

    for stock in stock_filter.iter() {
        let start_px = 0.0;
        let end_px = None;
        let tick_size = 0.01;

        stock_to_lob.insert(
            ArrayString8::from(stock).unwrap(),
            StockLOB {
                bid: L2BookBuilder::new(start_px, end_px, tick_size),
                ask: L2BookBuilder::new(start_px, end_px, tick_size),
            },
        );
    }

    for msg in stream {
        let tick0 = tick_counter::start();
        // ---------------------------------------------------------------------
        l2_from_itch.apply_message(&msg.unwrap().body, |stock, side, l2_delta| {
            if let Some(lob) = stock_to_lob.get_mut(stock) {
                match side {
                    itchy::Side::Buy => {
                        lob.bid.apply_l2_deltas(std::slice::from_ref(l2_delta));
                    }
                    itchy::Side::Sell => {
                        lob.ask.apply_l2_deltas(std::slice::from_ref(l2_delta));
                    }
                }

                let b0 = match lob.bid.book().levels().get(0) {
                    Some(b0) => *b0,
                    None => return,
                };

                let a0 = match lob.ask.book().levels().get(0) {
                    Some(a0) => *a0,
                    None => return,
                };

                let stock = *stock;

                assert_lt!(b0, a0);

                let async_task = Invocable::new(move || {
                    println!("stock=[{}], bids=[{}], asks=[{}]", stock, b0, a0);
                });

                let mut item = EventMessage::Event(async_task);
                while let Err(PushError::Full(i)) = producer.push(item) {
                    // println!("MarketData queue is full!");
                    item = i;
                    continue;
                }
            }
        });
        // ---------------------------------------------------------------------
        let tick1 = tick_counter::start();

        let async_task = Invocable::new(move || {
            println!(
                "latency=[{}]",
                ((tick1 - tick0) as f64 * counter_accuracy).round() as usize,
            );
        });

        let mut item = EventMessage::Event(async_task);
        while let Err(PushError::Full(i)) = producer.push(item) {
            // println!("MarketData queue is full!");
            item = i;
            continue;
        }
    }

    while let Err(_) = producer.push(EventMessage::Stop) {}
}

fn main() {
    let (producer, consumer) = RingBuffer::new(2_usize.pow(32));

    std::thread::scope(move |s| {
        s.spawn(move || {
            async_task(consumer);
        });

        limit_order_book_task(producer);
    });
}
