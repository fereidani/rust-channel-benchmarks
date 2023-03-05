use futures::channel::mpsc;
use futures::executor::{block_on, ThreadPool};
use futures::{future, stream, SinkExt, StreamExt, join};
use futures::stream::FuturesUnordered;

std::include!("z_types.rs");
std::include!("settings.rs");
std::include!("z_run.rs");

fn seq_unbounded<T: BenchType + 'static>() {
    block_on(async {
        let (tx, rx) = mpsc::unbounded();
        for i in 1..MESSAGES + 1 {
            tx.unbounded_send(T::new(i)).unwrap();
        }
        drop(tx);

        rx.for_each(|t| {
            t.test();
            future::ready(())
        })
        .await
    });
}

fn seq_bounded<T: BenchType + 'static>(cap: usize) {
    let (mut tx, rx) = mpsc::channel(cap);
    block_on(async {
        for i in 1..MESSAGES + 1 {
            tx.try_send(T::new(i)).unwrap();
        }
        drop(tx);

        rx.for_each(|t| {
            t.test();
            future::ready(())
        })
        .await
    });
}

fn spsc_unbounded<T: BenchType + 'static>() {
    block_on(async {
        let (mut tx, rx) = mpsc::unbounded();

        let jobs = FuturesUnordered::new();
        jobs.push(async move {
            tx.send_all(&mut stream::iter((1..MESSAGES + 1).map(T::new).map(Ok)))
                .await
                .unwrap()
        });

        let sender = jobs.collect::<Vec<_>>();
        join!(sender, rx.for_each(|t| {
            t.test();
            future::ready(())
        }))
    });
}

fn spsc_bounded<T: BenchType + 'static>(cap: usize) {
    block_on(async {
        let (mut tx, rx) = mpsc::channel(cap);

        let jobs = FuturesUnordered::new();
        jobs.push(async move {
            tx.send_all(&mut stream::iter((1..MESSAGES + 1).map(T::new).map(Ok)))
                .await
                .unwrap()
        });

        let sender = jobs.collect::<Vec<_>>();
        join!(sender, rx.for_each(|t| {
            t.test();
            future::ready(())
        }))
    });
}

fn mpsc_unbounded<T: BenchType + 'static>() {
    block_on(async {
        let (tx, rx) = mpsc::unbounded();

        let jobs = FuturesUnordered::new();
        for _ in 0..THREADS {
            let mut tx = tx.clone();
            jobs.push(async move {
                tx.send_all(&mut stream::iter(
                    (1..MESSAGES / THREADS + 1).map(T::new).map(Ok),
                ))
                .await
                .unwrap()
            });
        }
        drop(tx);

        let sender = jobs.collect::<Vec<_>>();
        join!(sender, rx.for_each(|t| {
            t.test();
            future::ready(())
        }))
    });
}

fn mpsc_bounded<T: BenchType + 'static>(cap: usize) {
    block_on(async {
        let (tx, rx) = mpsc::channel(cap);
        let jobs = FuturesUnordered::new();
        for _ in 0..THREADS {
            let mut tx = tx.clone();
            jobs.push(async move {
                tx.send_all(&mut stream::iter(
                    (1..MESSAGES / THREADS + 1).map(T::new).map(Ok),
                ))
                    .await
                    .unwrap()
            });
        }
        drop(tx);
        let sender = jobs.collect::<Vec<_>>();
        join!(sender, rx.for_each(|t| {
            t.test();
            future::ready(())
        }))
    });
}

fn main() {
    println!("futures-channel");

    run!("bounded0_mpsc(empty)", mpsc_bounded::<BenchEmpty>(0));
    run!("bounded0_spsc(empty)", spsc_bounded::<BenchEmpty>(0));

    run!("bounded1_mpsc(empty)", mpsc_bounded::<BenchEmpty>(1));
    run!("bounded1_spsc(empty)", spsc_bounded::<BenchEmpty>(1));

    run!("bounded_mpsc(empty)", mpsc_bounded::<BenchEmpty>(MESSAGES));
    run!("bounded_seq(empty)", seq_bounded::<BenchEmpty>(MESSAGES));
    run!("bounded_spsc(empty)", spsc_bounded::<BenchEmpty>(MESSAGES));

    run!("unbounded_mpsc(empty)", mpsc_unbounded::<BenchEmpty>());
    run!("unbounded_seq(empty)", seq_unbounded::<BenchEmpty>());
    run!("unbounded_spsc(empty)", spsc_unbounded::<BenchEmpty>());

    run!("bounded0_mpsc(usize)", mpsc_bounded::<BenchUsize>(0));
    run!("bounded0_spsc(usize)", spsc_bounded::<BenchUsize>(0));

    run!("bounded1_mpsc(usize)", mpsc_bounded::<BenchUsize>(1));
    run!("bounded1_spsc(usize)", spsc_bounded::<BenchUsize>(1));

    run!("bounded_mpsc(usize)", mpsc_bounded::<BenchUsize>(MESSAGES));
    run!("bounded_seq(usize)", seq_bounded::<BenchUsize>(MESSAGES));
    run!("bounded_spsc(usize)", spsc_bounded::<BenchUsize>(MESSAGES));

    run!("unbounded_mpsc(usize)", mpsc_unbounded::<BenchUsize>());
    run!("unbounded_seq(usize)", seq_unbounded::<BenchUsize>());
    run!("unbounded_spsc(usize)", spsc_unbounded::<BenchUsize>());

    run!("bounded0_mpsc(big)", mpsc_bounded::<BenchFixedArray>(0));
    run!("bounded0_spsc(big)", spsc_bounded::<BenchFixedArray>(0));

    run!("bounded1_mpsc(big)", mpsc_bounded::<BenchFixedArray>(1));
    run!("bounded1_spsc(big)", spsc_bounded::<BenchFixedArray>(1));

    run!(
        "bounded_mpsc(big)",
        mpsc_bounded::<BenchFixedArray>(MESSAGES)
    );
    run!("bounded_seq(big)", seq_bounded::<BenchFixedArray>(MESSAGES));
    run!(
        "bounded_spsc(big)",
        spsc_bounded::<BenchFixedArray>(MESSAGES)
    );

    run!("unbounded_mpsc(big)", mpsc_unbounded::<BenchFixedArray>());
    run!("unbounded_seq(big)", seq_unbounded::<BenchFixedArray>());
    run!("unbounded_spsc(big)", spsc_unbounded::<BenchFixedArray>());
}
