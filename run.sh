#!/bin/bash
set -euxo pipefail
IFS=$'\n\t'
cd "$(dirname "$0")"

#rm -rf *.csv

cargo run --release --bin mpsc | tee mpsc.csv
cargo run --release --bin futures-channel | tee futures-channel.csv
cargo run --release --bin flume | tee flume.csv
cargo run --release --bin flume_async | tee flume_async.csv
cargo run --release --bin crossbeam-channel | tee crossbeam-channel.csv
cargo run --release --bin async-channel | tee async-channel.csv
cargo run --release --bin kanal | tee kanal.csv
cargo run --release --bin kanal-async | tee kanal-async.csv

go run go.go | tee go.csv

./plot.py ./*.csv
