#!/bin/bash
set -euxo pipefail
IFS=$'\n\t'
cd "$(dirname "$0")"

cargo run --release --bin kanal | tee target/kanal.csv
cargo run --release --bin kanal-async | tee target/kanal-async.csv

./plot.py ./target/*.csv
