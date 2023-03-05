use local_sync::mpsc;
use local_sync::mpsc::unbounded::{Rx, Tx};

std::include!("z_types.rs");
std::include!("settings.rs");
std::include!("z_run.rs");

async fn seq_unbounded<T: BenchType + 'static>() {
	let (tx, mut rx) = mpsc::unbounded::channel::<T>();
	for i in 1..MESSAGES + 1 {
		tx.send(T::new(i)).unwrap();
	}
	drop(tx);
	while let Some(item) = rx.recv().await {
		item.test();
	}
}

async fn seq_bounded<T: BenchType + 'static>(cap: usize) {
	let (tx, mut rx) = mpsc::bounded::channel(cap);
	for i in 1..MESSAGES + 1 {
		tx.send(T::new(i)).await.unwrap();
	}
	drop(tx);
	while let Some(item) = rx.recv().await {
		item.test();
	}
}

async fn spsc_unbounded<T: BenchType + 'static>() {
	let (tx, mut rx) = mpsc::unbounded::channel::<T>();

	monoio::spawn(async move {
		for i in 1..MESSAGES + 1 {
			tx.send(T::new(i)).unwrap();
		}
	});

	while let Some(item) = rx.recv().await {
		item.test();
	}
}

async fn spsc_bounded<T: BenchType + 'static>(cap: usize) {
	let (tx, mut rx) = mpsc::bounded::channel(cap);

	monoio::spawn(async move {
		for i in 1..MESSAGES + 1 {
			tx.send(T::new(i)).await.unwrap();
		}
	});

	while let Some(item) = rx.recv().await {
		item.test();
	}
}

async fn mpsc_unbounded<T: BenchType + 'static>() {
	let (tx, mut rx) = mpsc::unbounded::channel();

	for _ in 0..THREADS {
		let tx = tx.clone();
		monoio::spawn(async move {
			for i in 1..MESSAGES + 1 {
				tx.send(T::new(i)).unwrap();
			}
		});
	}
	drop(tx);

	while let Some(item) = rx.recv().await {
		item.test();
	}
}

async fn mpsc_bounded<T: BenchType + 'static>(cap: usize) {
	let (tx, mut rx) = mpsc::bounded::channel(cap);

	for _ in 0..THREADS {
		let tx = tx.clone();
		monoio::spawn(async move {
			for i in 1..MESSAGES + 1 {
				tx.send(T::new(i)).await.unwrap();
			}
		});
	}
	drop(tx);

	while let Some(item) = rx.recv().await {
		item.test();
	}
}

#[monoio::main]
async fn main() {
	println!("local-sync");

	run_async!("bounded1_mpsc(empty)", mpsc_bounded::<BenchEmpty>(1));
	run_async!("bounded1_spsc(empty)", spsc_bounded::<BenchEmpty>(1));

	run_async!("bounded_mpsc(empty)", mpsc_bounded::<BenchEmpty>(MESSAGES));
	run_async!("bounded_seq(empty)", seq_bounded::<BenchEmpty>(MESSAGES));
	run_async!("bounded_spsc(empty)", spsc_bounded::<BenchEmpty>(MESSAGES));

	run_async!("unbounded_mpsc(empty)", mpsc_unbounded::<BenchEmpty>());
	run_async!("unbounded_seq(empty)", seq_unbounded::<BenchEmpty>());
	run_async!("unbounded_spsc(empty)", spsc_unbounded::<BenchEmpty>());

	run_async!("bounded1_mpsc(usize)", mpsc_bounded::<BenchUsize>(1));
	run_async!("bounded1_spsc(usize)", spsc_bounded::<BenchUsize>(1));

	run_async!("bounded_mpsc(usize)", mpsc_bounded::<BenchUsize>(MESSAGES));
	run_async!("bounded_seq(usize)", seq_bounded::<BenchUsize>(MESSAGES));
	run_async!("bounded_spsc(usize)", spsc_bounded::<BenchUsize>(MESSAGES));

	run_async!("unbounded_mpsc(usize)", mpsc_unbounded::<BenchUsize>());
	run_async!("unbounded_seq(usize)", seq_unbounded::<BenchUsize>());
	run_async!("unbounded_spsc(usize)", spsc_unbounded::<BenchUsize>());

	run_async!("bounded1_mpsc(big)", mpsc_bounded::<BenchFixedArray>(1));
	run_async!("bounded1_spsc(big)", spsc_bounded::<BenchFixedArray>(1));

	run_async!(
        "bounded_mpsc(big)",
        mpsc_bounded::<BenchFixedArray>(MESSAGES)
    );
	run_async!("bounded_seq(big)", seq_bounded::<BenchFixedArray>(MESSAGES));
	run_async!(
        "bounded_spsc(big)",
        spsc_bounded::<BenchFixedArray>(MESSAGES)
    );

	run_async!("unbounded_mpsc(big)", mpsc_unbounded::<BenchFixedArray>());
	run_async!("unbounded_seq(big)", seq_unbounded::<BenchFixedArray>());
	run_async!("unbounded_spsc(big)", spsc_unbounded::<BenchFixedArray>());
}
