use bus::Bus;

mod message;

std::include!("settings.in");

fn seq(cap: usize) {
    let mut tx = Bus::new(cap);
    let mut rx = tx.add_rx();

    for i in 0..MESSAGES {
        tx.broadcast(message::new(i));
    }

    for _ in 0..MESSAGES {
        rx.recv().unwrap();
    }
}

fn spsc(cap: usize) {
    let mut tx = Bus::new(cap);
    let mut rx = tx.add_rx();

    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            for i in 0..MESSAGES {
                tx.broadcast(message::new(i));
            }
        });

        for _ in 0..MESSAGES {
            rx.recv().unwrap();
        }
    })
    .unwrap();
}

fn main() {
    macro_rules! run {
        ($name:expr, $f:expr) => {
            let now = ::std::time::Instant::now();
            $f;
            let elapsed = now.elapsed();
            println!("{},{}", $name, elapsed.as_nanos());
        };
    }

    println!("bus");

    run!("bounded1_spsc", spsc(1));

    run!("bounded_seq", seq(MESSAGES));
    run!("bounded_spsc", spsc(MESSAGES));
}
