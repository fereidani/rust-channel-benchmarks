# Rust Channel Benchmarks
This is a highly modified fork of the crossbeam-channel benchmarks. to keep track of Kanal library stats in comparison with other competitors.
### Tests

* `seq`: A single thread sends `N` messages. Then it receives `N` messages.
* `spsc`: One thread sends `N` messages. Another thread receives `N` messages.
* `mpsc`: `T` threads send `N / T` messages each. One thread receives `N` messages.
* `mpmc`: `T` threads send `N / T` messages each. `T` other threads receive `N / T` messages each.
* `select_rx`: `T` threads send `N / T` messages each into a separate channel. Another thread receives `N` messages by selecting over the `T` channels.
* `select_both`: `T` threads send `N / T` messages each by selecting over `T` channels. `T` other threads receive `N / T` messages each by selecting over the `T` channels.

Default configuration:

- `N = 5000000`
- `T = 4`

### Running

Runs benchmarks, stores results into `*.txt` files, and generates `plot.png`:

```bash
# Results will be saved in `target`.
./run.sh
```

Dependencies:

- Rust (latest)
- Go
- Bash
- Python 2
- pygal
- cairosvg

### Results

Machine: AMD Ryzen Threadripper 2950X 16-Core Processor
Rust: `rustc 1.62.0`
Go: `go version go1.18.3 linux/amd64`
July 14 2022

![Benchmark bounded channel with size 0](https://i.imgur.com/vEBirUw.png)
![Benchmark bounded channel with size 1](https://i.imgur.com/iDETIAK.png)
![Benchmark bounded channel with size n](https://i.imgur.com/qdjXzyh.png)
![Benchmark unbounded channel](https://i.imgur.com/idxEm3k.png)