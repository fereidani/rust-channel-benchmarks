fn spsc<T: BenchType>(cap: Option<usize>) {
    let (tx, rx) = new(cap);

    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            for i in 1..MESSAGES + 1 {
                tx.send(T::new(i)).unwrap();
            }
        });

        for _ in 0..MESSAGES {
            rx.recv().unwrap().test()
        }
    })
    .unwrap();
}
