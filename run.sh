#!/bin/sh


export VCD_FILE=$1
export CONFIG_FILE=$2

mkdir -p out

rm -f out/vcd.json out/wave.json
./venv/bin/python3 scripts/main.py --vcd_file $VCD_FILE > out/vcd.json
cargo run -- --vcd-file out/vcd.json --config-file $CONFIG_FILE | tee out/wave.json
wavedrom-cli --input out/wave.json --svg out/wave.svg
