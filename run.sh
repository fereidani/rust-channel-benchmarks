#!/bin/bash
export RUSTFLAGS="-C target-cpu=native"
set -euxo pipefail
IFS=$'\n\t'
SLEEP_SEC=2
cd "$(dirname "$0")"

rm -rf kanal2
git clone https://github.com/fereidani/kanal/ kanal2 # hack to be able to reimport kanal with cargo

cargo clean
cargo update

mkdir -p target

cargo build --release --bin mpsc
cargo build --release --bin futures-channel
cargo build --release --bin flume
cargo build --release --bin flume-async
cargo build --release --bin crossbeam-channel
cargo build --release --bin async-channel
cargo build --release --bin kanal
cargo build --release --bin kanal-async
cargo build --release --bin kanal-std-mutex
cargo build --release --bin kanal-std-mutex-async
go build -o target/release/go_bench go.go


sleep $SLEEP_SEC
./target/release/mpsc | tee target/mpsc.csv
sleep $SLEEP_SEC
./target/release/futures-channel | tee target/futures-channel.csv
sleep $SLEEP_SEC
./target/release/flume | tee target/flume.csv
sleep $SLEEP_SEC
./target/release/flume-async | tee target/flume_async.csv
sleep $SLEEP_SEC
./target/release/crossbeam-channel | tee target/crossbeam-channel.csv
sleep $SLEEP_SEC
./target/release/async-channel | tee target/async-channel.csv
sleep $SLEEP_SEC
./target/release/kanal | tee target/kanal.csv
sleep $SLEEP_SEC
./target/release/kanal-async | tee target/kanal-async.csv
sleep $SLEEP_SEC 
./target/release/kanal-std-mutex | tee target/kanal-std-mutex.csv
sleep $SLEEP_SEC
./target/release/kanal-std-mutex-async | tee target/kanal-std-mutex-async.csv
sleep $SLEEP_SEC 
./target/release/go_bench | tee target/go.csv

./plot.py target/*.csv

echo "Test Environment:"
uname -srvp
rustc --version
go version