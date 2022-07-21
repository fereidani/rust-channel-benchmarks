use async_channel::{bounded, unbounded, Receiver, Sender};

std::include!("settings.rs");
std::include!("z_types.rs");
std::include!("z_async_all.rs");
std::include!("z_run.rs");

fn new<T>(cap: Option<usize>) -> (Sender<T>, Receiver<T>) {
    match cap {
        None => unbounded(),
        Some(cap) => bounded(cap),
    }
}

#[tokio::main]
async fn main() {
    println!("async-channel");

    run_async!("bounded1_mpmc(empty)", mpmc::<BenchEmpty>(Some(1)));
    run_async!("bounded1_mpsc(empty)", mpsc::<BenchEmpty>(Some(1)));
    run_async!("bounded1_spsc(empty)", spsc::<BenchEmpty>(Some(1)));
    run_async!("bounded_mpmc(empty)", mpmc::<BenchEmpty>(Some(MESSAGES)));
    run_async!("bounded_mpsc(empty)", mpsc::<BenchEmpty>(Some(MESSAGES)));
    run_async!("bounded_seq(empty)", seq::<BenchEmpty>(Some(MESSAGES)));
    run_async!("bounded_spsc(empty)", spsc::<BenchEmpty>(Some(MESSAGES)));
    run_async!("unbounded_mpmc(empty)", mpmc::<BenchEmpty>(None));
    run_async!("unbounded_mpsc(empty)", mpsc::<BenchEmpty>(None));
    run_async!("unbounded_seq(empty)", seq::<BenchEmpty>(None));
    run_async!("unbounded_spsc(empty)", spsc::<BenchEmpty>(None));

    run_async!("bounded1_mpmc(usize)", mpmc::<BenchUsize>(Some(1)));
    run_async!("bounded1_mpsc(usize)", mpsc::<BenchUsize>(Some(1)));
    run_async!("bounded1_spsc(usize)", spsc::<BenchUsize>(Some(1)));
    run_async!("bounded_mpmc(usize)", mpmc::<BenchUsize>(Some(MESSAGES)));
    run_async!("bounded_mpsc(usize)", mpsc::<BenchUsize>(Some(MESSAGES)));
    run_async!("bounded_seq(usize)", seq::<BenchUsize>(Some(MESSAGES)));
    run_async!("bounded_spsc(usize)", spsc::<BenchUsize>(Some(MESSAGES)));
    run_async!("unbounded_mpmc(usize)", mpmc::<BenchUsize>(None));
    run_async!("unbounded_mpsc(usize)", mpsc::<BenchUsize>(None));
    run_async!("unbounded_seq(usize)", seq::<BenchUsize>(None));
    run_async!("unbounded_spsc(usize)", spsc::<BenchUsize>(None));

    run_async!("bounded1_mpmc(big)", mpmc::<BenchFixedArray>(Some(1)));
    run_async!("bounded1_mpsc(big)", mpsc::<BenchFixedArray>(Some(1)));
    run_async!("bounded1_spsc(big)", spsc::<BenchFixedArray>(Some(1)));
    run_async!("bounded_mpmc(big)", mpmc::<BenchFixedArray>(Some(MESSAGES)));
    run_async!("bounded_mpsc(big)", mpsc::<BenchFixedArray>(Some(MESSAGES)));
    run_async!("bounded_seq(big)", seq::<BenchFixedArray>(Some(MESSAGES)));
    run_async!("bounded_spsc(big)", spsc::<BenchFixedArray>(Some(MESSAGES)));
    run_async!("unbounded_mpmc(big)", mpmc::<BenchFixedArray>(None));
    run_async!("unbounded_mpsc(big)", mpsc::<BenchFixedArray>(None));
    run_async!("unbounded_seq(big)", seq::<BenchFixedArray>(None));
    run_async!("unbounded_spsc(big)", spsc::<BenchFixedArray>(None));
}
