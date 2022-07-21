fn mpmc<T: BenchType>(cap: Option<usize>) {
    let (tx, rx) = new(cap);

    crossbeam::scope(|scope| {
        for _ in 0..THREADS {
            scope.spawn(|_| {
                for i in 1..MESSAGES / THREADS + 1 {
                    tx.send(T::new(i)).unwrap();
                }
            });
        }

        for _ in 0..THREADS {
            scope.spawn(|_| {
                for _ in 0..MESSAGES / THREADS {
                    rx.recv().unwrap().test()
                }
            });
        }
    })
    .unwrap();
}
