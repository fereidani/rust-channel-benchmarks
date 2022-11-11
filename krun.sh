#!/bin/bash
export RUSTFLAGS="-C target-cpu=native"
set -euxo pipefail
IFS=$'\n\t'
SLEEP_SEC=2
cd "$(dirname "$0")"

rm -rf kanal2
git clone https://github.com/fereidani/kanal/ kanal2

cargo build --release --bin kanal
cargo build --release --bin kanal-async


sleep $SLEEP_SEC
./target/release/kanal | tee target/kanal.csv
sleep $SLEEP_SEC
./target/release/kanal-async | tee target/kanal-async.csv

./plot.py target/*.csv

echo "Test Environment:"
uname -srvp
rustc --version
go version