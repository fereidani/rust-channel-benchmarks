#!/bin/bash
set -euxo pipefail
IFS=$'\n\t'
cd "$(dirname "$0")"

#rm -rf *.csv

mkdir -p target

cargo run --release --bin mpsc | tee target/mpsc.csv
cargo run --release --bin futures-channel | tee target/futures-channel.csv
cargo run --release --bin flume | tee target/flume.csv
cargo run --release --bin flume-async | tee target/flume_async.csv
cargo run --release --bin crossbeam-channel | tee target/crossbeam-channel.csv
cargo run --release --bin async-channel | tee target/async-channel.csv
cargo run --release --bin kanal | tee target/kanal.csv
cargo run --release --bin kanal-async | tee target/kanal-async.csv

go run go.go | tee target/go.csv

./plot.py target/*.csv
