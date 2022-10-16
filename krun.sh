#!/bin/bash
set -euxo pipefail
IFS=$'\n\t'
SLEEP_SEC=2
cd "$(dirname "$0")"

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