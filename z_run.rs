use std::time::Duration;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! run {
    ($name:expr, $f:expr) => {
        let mut sum_elapsed = Duration::new(0, 0);
        let mut count = 0u32;
        loop {
            let now = Instant::now();
            $f;
            let elapsed = now.elapsed();
            sum_elapsed += elapsed;
            count += 1;
            if sum_elapsed >= Duration::from_millis(MIN_BENCH_TIME) {
                break;
            }
        }
        println!(
            "{},{},{}",
            $name,
            (sum_elapsed / count).as_nanos(),
            ((count as f64 * MESSAGES as f64) / sum_elapsed.as_secs_f64()).round()
        );
    };
}

#[allow(unused_macros)]
macro_rules! run_async {
    ($name:expr, $f:expr) => {
        let mut sum_elapsed = Duration::new(0, 0);
        let mut count = 0u32;
        loop {
            let now = Instant::now();
            $f.await;
            let elapsed = now.elapsed();
            sum_elapsed += elapsed;
            count += 1;
            if sum_elapsed >= Duration::from_millis(MIN_BENCH_TIME) {
                break;
            }
        }
        println!(
            "{},{},{}",
            $name,
            (sum_elapsed / count).as_nanos(),
            ((count as f64 * MESSAGES as f64) / sum_elapsed.as_secs_f64()).round()
        );
    };
}
