#!/bin/bash
set -euxo pipefail
IFS=$'\n\t'
SLEEP_SEC=5
cd "$(dirname "$0")"

#rm -rf *.csv

uname -srvp
rustc --version
go version

mkdir -p target

cargo build --release --bin kanal
cargo build --release --bin kanal-async
go build -o target/release/go_bench go.go


sleep $SLEEP_SEC
./target/release/kanal | tee target/kanal.csv
sleep $SLEEP_SEC
./target/release/kanal-async | tee target/kanal-async.csv

./plot.py target/*.csv
