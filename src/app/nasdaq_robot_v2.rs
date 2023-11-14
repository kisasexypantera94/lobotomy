extern crate lobotomy;

use std::time::Instant;

use lobotomy::common::communication::EventMessage;
use lobotomy::common::types::L2Delta;
use lobotomy::common::StackInvocable;
use lobotomy::nasdaq::v2::ItchL2BookBuilder;
use lobotomy::nasdaq::Price4Wrapper;

use itchy::{Body, Price4};
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
    let counter_accuracy = calibrate_tick_counter();

    // https://emi.nasdaq.com/ITCH/Nasdaq%20ITCH/#:~:text=25%20AM%20%20%204075649457-,08302019.NASDAQ_ITCH50.gz,-8/31/2019
    let filename = "/Users/dvgr/dev/resources/08302019.NASDAQ_ITCH50";
    let stream = itchy::MessageStream::from_file(filename).unwrap();

    let mut l2_book_builder = ItchL2BookBuilder::new(Price4Wrapper(Price4::from(100)));

    let start = Instant::now();
    let mut num_packets = 0;
    let mut sum = 0;

    for msg in stream {
        let msg = msg.unwrap();

        num_packets += 1;
        let tick0 = tick_counter::start();
        // ---------------------------------------------------------------------
        l2_book_builder.apply_message(&msg);
        // ---------------------------------------------------------------------
        let tick1 = tick_counter::stop();

        sum += (tick1 - tick0);

        // let async_task = Invocable::new(move || {
        //     println!(
        //         "latency=[{}]",
        //         ((tick1 - tick0) as f64 * counter_accuracy).round() as usize,
        //     );
        // });

        // let mut item = EventMessage::Event(async_task);
        // while let Err(PushError::Full(i)) = async_producer.push(item) {
        //     // println!("MarketData queue is full!");
        //     item = i;
        //     continue;
        // }
    }

    let end = Instant::now();
    let sum = sum as f64 * counter_accuracy;

    println!("Mean latency: {}", (end - start).as_nanos() / num_packets);
    println!(
        "Mean latency: {}, {}, {}",
        sum,
        num_packets,
        sum / num_packets as f64
    );

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
