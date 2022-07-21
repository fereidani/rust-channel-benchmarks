fn seq<T: BenchType>(cap: Option<usize>) {
    let (tx, rx) = new(cap);

    for i in 1..MESSAGES + 1 {
        tx.send(T::new(i)).unwrap();
    }

    for _ in 0..MESSAGES {
        rx.recv().unwrap().test()
    }
}
