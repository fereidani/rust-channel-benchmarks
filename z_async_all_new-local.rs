async fn mpmc<T: BenchType + 'static>(cap: usize) {
    let (tx, mut rx) = new(cap);
    let mut list = Vec::new();
    for _ in 0..THREADS {
        let tx = tx.clone();
        let h = monoio::spawn(async move {
            for i in 1..MESSAGES / THREADS + 1 {
                tx.send(T::new(i)).await.unwrap();
            }
        });
        list.push(h);
    }

    for _ in 0..THREADS {
        let rx = rx.clone();
        let h = monoio::spawn(async move {
            for _ in 0..MESSAGES / THREADS {
                rx.recv().await.unwrap().test()
            }
        });
        list.push(h);
    }

    for h in list {
        h.await;
    }
}

async fn mpmc_unbound<T: BenchType + 'static>() {
    let (tx, mut rx) = new_unbound();
    let mut list = Vec::new();
    for _ in 0..THREADS {
        let tx = tx.clone();
        let h = monoio::spawn(async move {
            for i in 1..MESSAGES / THREADS + 1 {
                tx.send(T::new(i)).await.unwrap();
            }
        });
        list.push(h);
    }

    for _ in 0..THREADS {
        let rx = rx.clone();
        let h = monoio::spawn(async move {
            for _ in 0..MESSAGES / THREADS {
                rx.recv().await.unwrap().test()
            }
        });
        list.push(h);
    }

    for h in list {
        h.await;
    }
}

async fn mpsc<T: BenchType + 'static>(cap: usize) {
    let (tx, mut rx) = new(cap);
    let mut list = Vec::new();

    for _ in 0..THREADS {
        let tx = tx.clone();
        let h = monoio::spawn(async move {
            for i in 1..MESSAGES / THREADS + 1 {
                tx.send(T::new(i)).await.unwrap();
            }
        });
        list.push(h);
    }

    list.push(monoio::spawn(async move {
        for _ in 0..MESSAGES {
            rx.recv().await.unwrap().test()
        }
    }));

    for h in list {
        h.await;
    }
}

async fn mpsc_unbound<T: BenchType + 'static>() {
    let (tx, mut rx) = new_unbound;
    let mut list = Vec::new();

    for _ in 0..THREADS {
        let tx = tx.clone();
        let h = monoio::spawn(async move {
            for i in 1..MESSAGES / THREADS + 1 {
                tx.send(T::new(i)).await.unwrap();
            }
        });
        list.push(h);
    }

    list.push(monoio::spawn(async move {
        for _ in 0..MESSAGES {
            rx.recv().await.unwrap().test()
        }
    }));

    for h in list {
        h.await;
    }
}

async fn seq<T: BenchType + 'static>(cap: usize) {
    let (tx, mut rx) = new(cap);

    let h = monoio::spawn(async move {
        for i in 1..MESSAGES + 1 {
            tx.send(T::new(i)).await.unwrap();
        }

        for _ in 0..MESSAGES {
            rx.recv().await.unwrap().test()
        }
    });

    h.await;
}

async fn seq_unbound<T: BenchType + 'static>() {
    let (tx, mut rx) = new_unbound();

    let h = monoio::spawn(async move {
        for i in 1..MESSAGES + 1 {
            tx.send(T::new(i)).await.unwrap();
        }

        for _ in 0..MESSAGES {
            rx.recv().await.unwrap().test()
        }
    });

    h.await;
}

async fn spsc<T: BenchType + 'static>(cap: usize) {
    let (tx, mut rx) = new(cap);

    let htx = monoio::spawn(async move {
        for i in 1..MESSAGES + 1 {
            tx.send(T::new(i)).await.unwrap();
        }
    });
    let hrx = monoio::spawn(async move {
        for _ in 0..MESSAGES {
            rx.recv().await.unwrap().test()
        }
    });

    htx.await;
    hrx.await;
}

async fn spsc_unbound<T: BenchType + 'static>() {
    let (tx, mut rx) = new_unbound();

    let htx = monoio::spawn(async move {
        for i in 1..MESSAGES + 1 {
            tx.send(T::new(i)).await.unwrap();
        }
    });
    let hrx = monoio::spawn(async move {
        for _ in 0..MESSAGES {
            rx.recv().await.unwrap().test()
        }
    });

    htx.await;
    hrx.await;
}
