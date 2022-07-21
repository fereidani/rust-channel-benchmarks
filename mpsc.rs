use std::sync::mpsc;

std::include!("z_types.rs");
std::include!("settings.rs");
std::include!("z_run.rs");

fn seq_async<T: BenchType>() {
    let (tx, rx) = mpsc::channel();

    for i in 1..MESSAGES + 1 {
        tx.send(T::new(i)).unwrap();
    }

    for _ in 0..MESSAGES {
        rx.recv().unwrap().test();
    }
}

fn seq_sync<T: BenchType>(cap: usize) {
    let (tx, rx) = mpsc::sync_channel(cap);

    for i in 1..MESSAGES + 1 {
        tx.send(T::new(i)).unwrap();
    }

    for _ in 0..MESSAGES {
        rx.recv().unwrap().test();
    }
}

fn spsc_async<T: BenchType>() {
    let (tx, rx) = mpsc::channel();

    crossbeam::scope(|scope| {
        scope.spawn(move |_| {
            for i in 1..MESSAGES + 1 {
                tx.send(T::new(i)).unwrap();
            }
        });

        for _ in 0..MESSAGES {
            rx.recv().unwrap().test();
        }
    })
    .unwrap();
}

fn spsc_sync<T: BenchType>(cap: usize) {
    let (tx, rx) = mpsc::sync_channel(cap);

    crossbeam::scope(|scope| {
        scope.spawn(move |_| {
            for i in 1..MESSAGES + 1 {
                tx.send(T::new(i)).unwrap();
            }
        });

        for _ in 0..MESSAGES {
            rx.recv().unwrap().test();
        }
    })
    .unwrap();
}

fn mpsc_async<T: BenchType>() {
    let (tx, rx) = mpsc::channel();

    crossbeam::scope(|scope| {
        for _ in 0..THREADS {
            let tx = tx.clone();
            scope.spawn(move |_| {
                for i in 1..MESSAGES / THREADS + 1 {
                    tx.send(T::new(i)).unwrap();
                }
            });
        }

        for _ in 0..MESSAGES {
            rx.recv().unwrap().test();
        }
    })
    .unwrap();
}

fn mpsc_sync<T: BenchType>(cap: usize) {
    let (tx, rx) = mpsc::sync_channel(cap);

    crossbeam::scope(|scope| {
        for _ in 0..THREADS {
            let tx = tx.clone();
            scope.spawn(move |_| {
                for i in 1..MESSAGES / THREADS + 1 {
                    tx.send(T::new(i)).unwrap();
                }
            });
        }

        for _ in 0..MESSAGES {
            rx.recv().unwrap().test();
        }
    })
    .unwrap();
}

fn main() {
    println!("std::mpsc");
    run!("bounded0_mpsc(empty)", mpsc_sync::<BenchEmpty>(0));
    run!("bounded0_spsc(empty)", spsc_sync::<BenchEmpty>(0));

    run!("bounded1_mpsc(empty)", mpsc_sync::<BenchEmpty>(1));
    run!("bounded1_spsc(empty)", spsc_sync::<BenchEmpty>(1));

    run!("bounded_mpsc(empty)", mpsc_sync::<BenchEmpty>(MESSAGES));
    run!("bounded_seq(empty)", seq_sync::<BenchEmpty>(MESSAGES));
    run!("bounded_spsc(empty)", spsc_sync::<BenchEmpty>(MESSAGES));

    run!("unbounded_mpsc(empty)", mpsc_async::<BenchEmpty>());
    run!("unbounded_seq(empty)", seq_async::<BenchEmpty>());
    run!("unbounded_spsc(empty)", spsc_async::<BenchEmpty>());

    run!("bounded0_mpsc(usize)", mpsc_sync::<BenchUsize>(0));
    run!("bounded0_spsc(usize)", spsc_sync::<BenchUsize>(0));

    run!("bounded1_mpsc(usize)", mpsc_sync::<BenchUsize>(1));
    run!("bounded1_spsc(usize)", spsc_sync::<BenchUsize>(1));

    run!("bounded_mpsc(usize)", mpsc_sync::<BenchUsize>(MESSAGES));
    run!("bounded_seq(usize)", seq_sync::<BenchUsize>(MESSAGES));
    run!("bounded_spsc(usize)", spsc_sync::<BenchUsize>(MESSAGES));

    run!("unbounded_mpsc(usize)", mpsc_async::<BenchUsize>());
    run!("unbounded_seq(usize)", seq_async::<BenchUsize>());
    run!("unbounded_spsc(usize)", spsc_async::<BenchUsize>());

    run!("bounded0_mpsc(big)", mpsc_sync::<BenchFixedArray>(0));
    run!("bounded0_spsc(big)", spsc_sync::<BenchFixedArray>(0));

    run!("bounded1_mpsc(big)", mpsc_sync::<BenchFixedArray>(1));
    run!("bounded1_spsc(big)", spsc_sync::<BenchFixedArray>(1));

    run!("bounded_mpsc(big)", mpsc_sync::<BenchFixedArray>(MESSAGES));
    run!("bounded_seq(big)", seq_sync::<BenchFixedArray>(MESSAGES));
    run!("bounded_spsc(big)", spsc_sync::<BenchFixedArray>(MESSAGES));

    run!("unbounded_mpsc(big)", mpsc_async::<BenchFixedArray>());
    run!("unbounded_seq(big)", seq_async::<BenchFixedArray>());
    run!("unbounded_spsc(big)", spsc_async::<BenchFixedArray>());
}
