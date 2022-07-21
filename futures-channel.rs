use futures::channel::mpsc;
use futures::executor::{block_on, ThreadPool};
use futures::{future, stream, SinkExt, StreamExt};

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
    let pool = ThreadPool::new().unwrap();
    block_on(async {
        let (mut tx, rx) = mpsc::unbounded();

        pool.spawn_ok(async move {
            tx.send_all(&mut stream::iter((1..MESSAGES + 1).map(T::new).map(Ok)))
                .await
                .unwrap()
        });

        rx.for_each(|t| {
            t.test();
            future::ready(())
        })
        .await
    });
}

fn spsc_bounded<T: BenchType + 'static>(cap: usize) {
    let pool = ThreadPool::new().unwrap();
    block_on(async {
        let (mut tx, rx) = mpsc::channel(cap);

        pool.spawn_ok(async move {
            tx.send_all(&mut stream::iter((1..MESSAGES + 1).map(T::new).map(Ok)))
                .await
                .unwrap()
        });

        rx.for_each(|t| {
            t.test();
            future::ready(())
        })
        .await
    });
}

fn mpsc_unbounded<T: BenchType + 'static>() {
    let pool = ThreadPool::new().unwrap();
    block_on(async {
        let (tx, rx) = mpsc::unbounded();

        for _ in 0..THREADS {
            let mut tx = tx.clone();
            pool.spawn_ok(async move {
                tx.send_all(&mut stream::iter(
                    (1..MESSAGES / THREADS + 1).map(T::new).map(Ok),
                ))
                .await
                .unwrap()
            });
        }
        drop(tx);

        rx.for_each(|t| {
            t.test();
            future::ready(())
        })
        .await
    });
}

fn mpsc_bounded<T: BenchType + 'static>(cap: usize) {
    let pool = ThreadPool::new().unwrap();
    block_on(async {
        let (tx, rx) = mpsc::channel(cap);

        for _ in 0..THREADS {
            let mut tx = tx.clone();
            pool.spawn_ok(async move {
                tx.send_all(&mut stream::iter(
                    (1..MESSAGES / THREADS + 1).map(T::new).map(Ok),
                ))
                .await
                .unwrap()
            });
        }
        drop(tx);

        rx.for_each(|t| {
            t.test();
            future::ready(())
        })
        .await
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
