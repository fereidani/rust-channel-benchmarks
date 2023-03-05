# Rust Channel Benchmarks
This is a highly modified fork of the crossbeam-channel benchmarks. to keep track of Kanal library stats in comparison with other competitors.

### Running

Runs benchmarks, stores results into `*.csv` files in the target folder, and generates multiple png file for each test category:

```bash
# Results will be saved in `target`.
./run.sh
```
Run local benchmark. This used to determine performance in thread-per-core application.
```bash
# run local benchmark
./run-local.sh
```
> `local` in this context is stand for `Future + !Send + !Sync` (data are never send to other thread),
> mpsc/mpmc are done in concurrency instead of parallel.

Dependencies:

- Rust (latest)
- Go
- Git
- Bash
- libcairo2-dev
- Python
  - pygal
  - cairosvg
  - tk
  - PIL (pillow)

### Contributing

You can follow [community benchmarks](https://github.com/fereidani/rust-channel-benchmarks/issues?q=label%3Abenchmark), and also share your results by opening an issue with the format shown in [results](#Results) section.

### Benchmark Results
Results are based on how many messages can be passed in each scenario per second.

1. empty tests are those tests that are passing zero-sized message like notifications to receivers.
1. usize tests are those tests that are passing messages of register size to receivers.
1. big tests are those tests that are passing messages of 4x the size of the register to receivers, for example, 32 bytes(4x8) structure for x64 systems.

N/A means that the test subject can't perform the test because of its limitations, for example, some libraries don't have support for size 0 channels or MPMC.

Machine: `AMD Ryzen Threadripper 2950X 16-Core Processor`<br />
Rust: `rustc 1.65.0 (897e37553 2022-11-02)`<br />
Go: `go version go1.19.3 linux/amd64`<br />
OS (`uname -a`): `Linux 5.15.0-52-generic #58~20.04.1-Ubuntu SMP Thu Oct 13 13:09:46 UTC 2022 x86_64`<br />
Date: Nov 11, 2022

![Benchmarks](https://i.imgur.com/QK1UOyW.png)

#### Why in some tests async is much faster than sync?
It's because of Tokio's context-switching performance, like Golang, Tokio context-switch in the same thread to the next coroutine when the channel message is ready which is much cheaper than communicating between different threads, It's the same reason why async network applications usually perform better than sync implementations.
As channel size grows you see better performance in sync benchmarks because channel sender threads can push their data directly to the channel queue and don't need to wait for signals from receivers threads.