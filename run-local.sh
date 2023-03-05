#!/bin/bash
export RUSTFLAGS="-C target-cpu=native"
set -euxo pipefail
IFS=$'\n\t'
SLEEP_SEC=2
cd "$(dirname "$0")"

cargo clean
cargo update

mkdir -p target
rm target/*.csv || true

cargo build --release --bin kanal-local
cargo build --release --bin kanal-local-monoio
cargo build --release --bin flume-monoio
cargo build --release --bin futures-channel-local
cargo build --release --bin futures-channel-local-monoio
cargo build --release --bin local_sync

sleep $SLEEP_SEC
./target/release/kanal-local | tee target/kanal-local.csv
sleep $SLEEP_SEC
./target/release/kanal-local-monoio | tee target/kanal-local-monoio.csv
sleep $SLEEP_SEC
./target/release/flume-monoio | tee target/flume-monoio.csv
sleep $SLEEP_SEC
./target/release/futures-channel-local | tee target/futures-channel-local.csv
sleep $SLEEP_SEC
./target/release/futures-channel-local-monoio | tee target/futures-channel-local-monoio.csv
sleep $SLEEP_SEC
./target/release/local_sync | tee target/local_sync.csv

./plot.py target/*.csv

echo "Test Environment:"
uname -srvp
rustc --version