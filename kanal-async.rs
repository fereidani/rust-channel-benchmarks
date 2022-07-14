mod message;

use std::time::Duration;

use kanal::{bounded_async, unbounded_async, AsyncReceiver, AsyncSender};

std::include!("settings.in");

fn new<T>(cap: Option<usize>) -> (AsyncSender<T>, AsyncReceiver<T>) {
    match cap {
        None => unbounded_async(),
        Some(cap) => bounded_async(cap),
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
            true
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
    println!("kanal-async");

    //receive_from_queue().await; // OK
    //receive_directly_send_first().await;
    //receive_directly_recv_first().await;
    run!("bounded0_mpmc", mpmc(Some(0)));
    run!("bounded0_mpsc", mpsc(Some(0)));
    //run!("bounded0_select_both", select_both(Some(0)));
    //run!("bounded0_select_rx", select_rx(Some(0)));
    run!("bounded0_spsc", spsc(Some(0)));

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
