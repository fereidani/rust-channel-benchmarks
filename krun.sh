#!/bin/bash
set -euxo pipefail
IFS=$'\n\t'
cd "$(dirname "$0")"

cargo run --release --bin kanal | tee kanal.csv
cargo run --release --bin kanal-async | tee kanal-async.csv

./plot.py ./*.csv
