use crossbeam_channel::{bounded, unbounded, Receiver, Sender};

std::include!("settings.rs");
std::include!("z_types.rs");
std::include!("z_seq.rs");
std::include!("z_spsc.rs");
std::include!("z_mpsc.rs");
std::include!("z_mpmc.rs");
std::include!("z_run.rs");

fn new<T>(cap: Option<usize>) -> (Sender<T>, Receiver<T>) {
    match cap {
        None => unbounded(),
        Some(cap) => bounded(cap),
    }
}

fn main() {
    println!("crossbeam-channel");
    run!("bounded0_mpmc(empty)", mpmc::<BenchEmpty>(Some(0)));
    run!("bounded0_mpsc(empty)", mpsc::<BenchEmpty>(Some(0)));
    run!("bounded0_spsc(empty)", spsc::<BenchEmpty>(Some(0)));
    run!("bounded1_mpmc(empty)", mpmc::<BenchEmpty>(Some(1)));
    run!("bounded1_mpsc(empty)", mpsc::<BenchEmpty>(Some(1)));
    run!("bounded1_spsc(empty)", spsc::<BenchEmpty>(Some(1)));
    run!("bounded_mpmc(empty)", mpmc::<BenchEmpty>(Some(MESSAGES)));
    run!("bounded_mpsc(empty)", mpsc::<BenchEmpty>(Some(MESSAGES)));
    run!("bounded_seq(empty)", seq::<BenchEmpty>(Some(MESSAGES)));
    run!("bounded_spsc(empty)", spsc::<BenchEmpty>(Some(MESSAGES)));
    run!("unbounded_mpmc(empty)", mpmc::<BenchEmpty>(None));
    run!("unbounded_mpsc(empty)", mpsc::<BenchEmpty>(None));
    run!("unbounded_seq(empty)", seq::<BenchEmpty>(None));
    run!("unbounded_spsc(empty)", spsc::<BenchEmpty>(None));

    run!("bounded0_mpmc(usize)", mpmc::<BenchUsize>(Some(0)));
    run!("bounded0_mpsc(usize)", mpsc::<BenchUsize>(Some(0)));
    run!("bounded0_spsc(usize)", spsc::<BenchUsize>(Some(0)));
    run!("bounded1_mpmc(usize)", mpmc::<BenchUsize>(Some(1)));
    run!("bounded1_mpsc(usize)", mpsc::<BenchUsize>(Some(1)));
    run!("bounded1_spsc(usize)", spsc::<BenchUsize>(Some(1)));
    run!("bounded_mpmc(usize)", mpmc::<BenchUsize>(Some(MESSAGES)));
    run!("bounded_mpsc(usize)", mpsc::<BenchUsize>(Some(MESSAGES)));
    run!("bounded_seq(usize)", seq::<BenchUsize>(Some(MESSAGES)));
    run!("bounded_spsc(usize)", spsc::<BenchUsize>(Some(MESSAGES)));
    run!("unbounded_mpmc(usize)", mpmc::<BenchUsize>(None));
    run!("unbounded_mpsc(usize)", mpsc::<BenchUsize>(None));
    run!("unbounded_seq(usize)", seq::<BenchUsize>(None));
    run!("unbounded_spsc(usize)", spsc::<BenchUsize>(None));

    run!("bounded0_mpmc(big)", mpmc::<BenchFixedArray>(Some(0)));
    run!("bounded0_mpsc(big)", mpsc::<BenchFixedArray>(Some(0)));
    run!("bounded0_spsc(big)", spsc::<BenchFixedArray>(Some(0)));
    run!("bounded1_mpmc(big)", mpmc::<BenchFixedArray>(Some(1)));
    run!("bounded1_mpsc(big)", mpsc::<BenchFixedArray>(Some(1)));
    run!("bounded1_spsc(big)", spsc::<BenchFixedArray>(Some(1)));
    run!("bounded_mpmc(big)", mpmc::<BenchFixedArray>(Some(MESSAGES)));
    run!("bounded_mpsc(big)", mpsc::<BenchFixedArray>(Some(MESSAGES)));
    run!("bounded_seq(big)", seq::<BenchFixedArray>(Some(MESSAGES)));
    run!("bounded_spsc(big)", spsc::<BenchFixedArray>(Some(MESSAGES)));
    run!("unbounded_mpmc(big)", mpmc::<BenchFixedArray>(None));
    run!("unbounded_mpsc(big)", mpsc::<BenchFixedArray>(None));
    run!("unbounded_seq(big)", seq::<BenchFixedArray>(None));
    run!("unbounded_spsc(big)", spsc::<BenchFixedArray>(None));
}
