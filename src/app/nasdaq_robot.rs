extern crate lobotomy;

use lobotomy::common::communication::EventMessage;
use lobotomy::common::StackInvocable;
use lobotomy::nasdaq::ItchIntoL2Deltas;
use lobotomy::order_book::L2BookBuilder;

use heapless::spsc;

pub const QUEUE_SIZE: usize = 65536;
pub type SmallInvocable = StackInvocable<64>;

fn init_log() {
    fast_log::init(
        fast_log::Config::new()
            .console()
            .file("nasdaq_robot.log")
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

fn async_task(mut receiver: spsc::Consumer<EventMessage<SmallInvocable>, QUEUE_SIZE>) {
    loop {
        let msg = match receiver.dequeue() {
            Some(msg) => msg,
            None => {
                continue;
            }
        };

        match msg {
            EventMessage::Event(invocable) => invocable.invoke(),
            EventMessage::Stop() => return,
        }
    }
}

fn limit_order_book_task(mut sender: spsc::Producer<EventMessage<SmallInvocable>, QUEUE_SIZE>) {
    let counter_accuracy = calibrate_tick_counter();

    // https://emi.nasdaq.com/ITCH/Nasdaq%20ITCH/#:~:text=25%20AM%20%20%204075649457-,08302019.NASDAQ_ITCH50.gz,-8/31/2019
    let filename = "/Users/dvgr/dev/resources/08302019.NASDAQ_ITCH50";
    let stream = itchy::MessageStream::from_file(filename).unwrap();

    let mut l2_from_itch = ItchIntoL2Deltas::new(&vec!["GOOG".to_string()]);

    let start_px = 0.0;
    let end_px = None;
    let tick_size = 0.01;
    const LOB_SIZE: usize = 25;
    let mut bid_lob_builder = L2BookBuilder::<LOB_SIZE, true>::new(start_px, end_px, tick_size);
    let mut ask_lob_builder = L2BookBuilder::<LOB_SIZE, false>::new(start_px, end_px, tick_size);

    for msg in stream {
        let msg = msg.unwrap();
        let tick0 = tick_counter::start();
        l2_from_itch.apply_message(&msg.body, |_stock, side, l2_delta| match side {
            itchy::Side::Buy => {
                bid_lob_builder.apply_l2_deltas(std::slice::from_ref(l2_delta));
            }
            itchy::Side::Sell => {
                ask_lob_builder.apply_l2_deltas(std::slice::from_ref(l2_delta));
            }
        });
        let tick1 = tick_counter::start();

        let b0 = bid_lob_builder.book().levels().get(0).unwrap_or(&0.0);
        let a0 = ask_lob_builder.book().levels().get(0).unwrap_or(&0.0);

        let async_task = SmallInvocable::new(move || {
            println!(
                "latency=[{}], bids=[{:?}], asks=[{}]",
                ((tick1 - tick0) as f64 * counter_accuracy).round() as usize,
                b0,
                a0,
            );
        });

        let _ = sender.enqueue(EventMessage::Event(async_task));
    }

    let _ = sender.enqueue(EventMessage::Stop());
}

fn main() {
    init_log();

    let mut queue = spsc::Queue::<EventMessage<SmallInvocable>, QUEUE_SIZE>::new();
    let (sender, receiver) = queue.split();

    std::thread::scope(move |s| {
        s.spawn(move || {
            async_task(receiver);
        });

        limit_order_book_task(sender);
    });
}
