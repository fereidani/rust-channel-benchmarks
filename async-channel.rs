mod message;

use async_channel::{bounded, unbounded, Receiver, Sender};

std::include!("settings.in");

fn new<T>(cap: Option<usize>) -> (Sender<T>, Receiver<T>) {
    match cap {
        None => unbounded(),
        Some(cap) => bounded(cap),
    }
}

async fn mpmc(cap: Option<usize>) {
    let (tx, rx) = new(cap);
    let mut list = Vec::new();
    for _ in 0..THREADS {
        let tx = tx.clone();
        let h = tokio::spawn(async move {
            for i in 0..MESSAGES / THREADS {
                tx.send(message::new(i)).await.unwrap();
            }
        });
        list.push(h);
    }

    for _ in 0..THREADS {
        let rx = rx.clone();
        let h = tokio::spawn(async move {
            for _ in 0..MESSAGES / THREADS {
                rx.recv().await.unwrap();
            }
        });
        list.push(h);
    }

    for h in list {
        h.await.unwrap();
    }
}

async fn mpsc(cap: Option<usize>) {
    let (tx, rx) = new(cap);
    let mut list = Vec::new();

    for _ in 0..THREADS {
        let tx = tx.clone();
        let h = tokio::spawn(async move {
            for i in 0..MESSAGES / THREADS {
                tx.send(message::new(i)).await.unwrap();
            }
        });
        list.push(h);
    }

    for _ in 0..MESSAGES {
        rx.recv().await.unwrap();
    }
    for h in list {
        h.await.unwrap();
    }
}

async fn seq(cap: Option<usize>) {
    let (tx, rx) = new(cap);

    for i in 0..MESSAGES {
        tx.send(message::new(i)).await.unwrap();
    }

    for _ in 0..MESSAGES {
        rx.recv().await.unwrap();
    }
}

async fn spsc(cap: Option<usize>) {
    let (tx, rx) = new(cap);

    let h = tokio::spawn(async move {
        for i in 0..MESSAGES {
            tx.send(message::new(i)).await.unwrap();
        }
    });

    for _ in 0..MESSAGES {
        rx.recv().await.unwrap();
    }

    h.await.unwrap();
}

/*
fn select_rx(cap: Option<usize>) {
    let chans = (0..THREADS).map(|_| new(cap)).collect::<Vec<_>>();

    crossbeam::scope(|scope| {
        for (tx, _) in &chans {
            let tx = tx.clone();
            scope.spawn(move |_| {
                for i in 0..MESSAGES / THREADS {
                    tx.send(message::new(i)).await.unwrap();
                }
            });
        }

        for _ in 0..MESSAGES {
            let mut sel = Selector::new();
            for (_, rx) in &chans {
                sel.recv(rx);
            }
            let case = sel.select();
            let index = case.index();
            case.recv(&chans[index].1).await.unwrap();
        }
    })
    .await.unwrap();
}

fn select_both(cap: Option<usize>) {
    let chans = (0..THREADS).map(|_| new(cap)).collect::<Vec<_>>();

    crossbeam::scope(|scope| {
        for _ in 0..THREADS {
            tokio::spawn(async move {
                for i in 0..MESSAGES / THREADS {
                    let mut sel = Selector::new();
                    for (tx, _) in &chans {
                        sel.send(tx);
                    }
                    let case = sel.select();
                    let index = case.index();
                    case.send(&chans[index].0, message::new(i)).await.unwrap();
                }
            });
        }

        for _ in 0..THREADS {
            tokio::spawn(async move {
                for _ in 0..MESSAGES / THREADS {
                    let mut sel = Select::new();
                    for (_, rx) in &chans {
                        sel.recv(rx);
                    }
                    let case = sel.select();
                    let index = case.index();
                    case.recv(&chans[index].1).await.unwrap();
                }
            });
        }
    })
    .await.unwrap();
}
 */

#[tokio::main]
async fn main() {
    macro_rules! run {
        ($name:expr, $f:expr) => {
            let now = ::std::time::Instant::now();
            $f.await;
            let elapsed = now.elapsed();
            println!("{},{}", $name, elapsed.as_nanos());
        };
    }

    println!("async-channel");
    //run!("bounded0_mpmc", mpmc(Some(0)));
    //run!("bounded0_mpsc", mpsc(Some(0)));
    //run!("bounded0_select_both", select_both(Some(0)));
    //run!("bounded0_select_rx", select_rx(Some(0)));
    //run!("bounded0_spsc", spsc(Some(0)));

    run!("bounded1_mpmc", mpmc(Some(1)));
    run!("bounded1_mpsc", mpsc(Some(1)));
    //run!("bounded1_select_both", select_both(Some(1)));
    //run!("bounded1_select_rx", select_rx(Some(1)));
    run!("bounded1_spsc", spsc(Some(1)));

    run!("bounded_mpmc", mpmc(Some(MESSAGES)));
    run!("bounded_mpsc", mpsc(Some(MESSAGES)));
    //run!("bounded_select_both", select_both(Some(MESSAGES)));
    //run!("bounded_select_rx", select_rx(Some(MESSAGES)));
    run!("bounded_seq", seq(Some(MESSAGES)));
    run!("bounded_spsc", spsc(Some(MESSAGES)));

    run!("unbounded_mpmc", mpmc(None));
    run!("unbounded_mpsc", mpsc(None));
    //run!("unbounded_select_both", select_both(None));
    //run!("unbounded_select_rx", select_rx(None));
    run!("unbounded_seq", seq(None));
    run!("unbounded_spsc", spsc(None));
}
