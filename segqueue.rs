use crossbeam::queue::SegQueue;
use std::thread;

mod message;

std::include!("settings.in");

fn seq() {
    let q = SegQueue::new();

    for i in 0..MESSAGES {
        q.push(message::new(i));
    }

    for _ in 0..MESSAGES {
        q.pop().unwrap();
    }
}

fn spsc() {
    let q = SegQueue::new();

    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            for i in 0..MESSAGES {
                q.push(message::new(i));
            }
        });

        for _ in 0..MESSAGES {
            loop {
                if q.pop().is_none() {
                    thread::yield_now();
                } else {
                    break;
                }
            }
        }
    })
    .unwrap();
}

fn mpsc() {
    let q = SegQueue::new();

    crossbeam::scope(|scope| {
        for _ in 0..THREADS {
            scope.spawn(|_| {
                for i in 0..MESSAGES / THREADS {
                    q.push(message::new(i));
                }
            });
        }

        for _ in 0..MESSAGES {
            loop {
                if q.pop().is_none() {
                    thread::yield_now();
                } else {
                    break;
                }
            }
        }
    })
    .unwrap();
}

fn mpmc() {
    let q = SegQueue::new();

    crossbeam::scope(|scope| {
        for _ in 0..THREADS {
            scope.spawn(|_| {
                for i in 0..MESSAGES / THREADS {
                    q.push(message::new(i));
                }
            });
        }

        for _ in 0..THREADS {
            scope.spawn(|_| {
                for _ in 0..MESSAGES / THREADS {
                    loop {
                        if q.pop().is_none() {
                            thread::yield_now();
                        } else {
                            break;
                        }
                    }
                }
            });
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

    println!("segqueue");
    run!("unbounded_mpmc", mpmc());
    run!("unbounded_mpsc", mpsc());
    run!("unbounded_seq", seq());
    run!("unbounded_spsc", spsc());
}
