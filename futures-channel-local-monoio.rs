use futures::{future, SinkExt, stream, StreamExt};
use futures::channel::mpsc;

std::include!("z_types.rs");
std::include!("settings.rs");
std::include!("z_run.rs");

async fn seq_unbounded<T: BenchType + 'static>() {
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
}

async fn seq_bounded<T: BenchType + 'static>(cap: usize) {
	let (mut tx, rx) = mpsc::channel(cap);

	for i in 1..MESSAGES + 1 {
		tx.try_send(T::new(i)).unwrap();
	}
	drop(tx);

	rx.for_each(|t| {
		t.test();
		future::ready(())
	}).await
}

async fn spsc_unbounded<T: BenchType + 'static>() {
	let (mut tx, rx) = mpsc::unbounded();

	monoio::spawn(async move {
		tx.send_all(&mut stream::iter((1..MESSAGES + 1).map(T::new).map(Ok)))
			.await
			.unwrap()
	});

	rx.for_each(|t| {
		t.test();
		future::ready(())
	}).await;
}

async fn spsc_bounded<T: BenchType + 'static>(cap: usize) {
	let (mut tx, rx) = mpsc::channel(cap);

	monoio::spawn(async move {
		tx.send_all(&mut stream::iter((1..MESSAGES + 1).map(T::new).map(Ok)))
			.await
			.unwrap()
	});

	rx.for_each(|t| {
		t.test();
		future::ready(())
	}).await;
}

async fn mpsc_unbounded<T: BenchType + 'static>() {
	let (tx, rx) = mpsc::unbounded();

	for _ in 0..THREADS {
		let mut tx = tx.clone();
		monoio::spawn(async move {
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
	}).await;
}

async fn mpsc_bounded<T: BenchType + 'static>(cap: usize) {
	let (tx, rx) = mpsc::channel(cap);
	for _ in 0..THREADS {
		let mut tx = tx.clone();
		monoio::spawn(async move {
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
	}).await;
}

#[monoio::main]
async fn main() {
	println!("futures-channel-local-monoio");

	run_async!("bounded0_mpsc(empty)", mpsc_bounded::<BenchEmpty>(0));
	run_async!("bounded0_spsc(empty)", spsc_bounded::<BenchEmpty>(0));

	run_async!("bounded1_mpsc(empty)", mpsc_bounded::<BenchEmpty>(1));
	run_async!("bounded1_spsc(empty)", spsc_bounded::<BenchEmpty>(1));

	run_async!("bounded_mpsc(empty)", mpsc_bounded::<BenchEmpty>(MESSAGES));
	run_async!("bounded_seq(empty)", seq_bounded::<BenchEmpty>(MESSAGES));
	run_async!("bounded_spsc(empty)", spsc_bounded::<BenchEmpty>(MESSAGES));

	run_async!("unbounded_mpsc(empty)", mpsc_unbounded::<BenchEmpty>());
	run_async!("unbounded_seq(empty)", seq_unbounded::<BenchEmpty>());
	run_async!("unbounded_spsc(empty)", spsc_unbounded::<BenchEmpty>());

	run_async!("bounded0_mpsc(usize)", mpsc_bounded::<BenchUsize>(0));
	run_async!("bounded0_spsc(usize)", spsc_bounded::<BenchUsize>(0));

	run_async!("bounded1_mpsc(usize)", mpsc_bounded::<BenchUsize>(1));
	run_async!("bounded1_spsc(usize)", spsc_bounded::<BenchUsize>(1));

	run_async!("bounded_mpsc(usize)", mpsc_bounded::<BenchUsize>(MESSAGES));
	run_async!("bounded_seq(usize)", seq_bounded::<BenchUsize>(MESSAGES));
	run_async!("bounded_spsc(usize)", spsc_bounded::<BenchUsize>(MESSAGES));

	run_async!("unbounded_mpsc(usize)", mpsc_unbounded::<BenchUsize>());
	run_async!("unbounded_seq(usize)", seq_unbounded::<BenchUsize>());
	run_async!("unbounded_spsc(usize)", spsc_unbounded::<BenchUsize>());

	run_async!("bounded0_mpsc(big)", mpsc_bounded::<BenchFixedArray>(0));
	run_async!("bounded0_spsc(big)", spsc_bounded::<BenchFixedArray>(0));

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
