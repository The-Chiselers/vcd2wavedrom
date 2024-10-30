#!/bin/sh

mkdir -p out

rm -f out/vcd.json out/wave.json
./venv/bin/python3 scripts/main.py --vcd_file ./examples/atomic_and.vcd > out/vcd.json
cargo run -- --vcd-file out/vcd.json --config-file examples/atomic_and.json | tee out/wave.json
wavedrom-cli --input out/wave.json --svg out/wave.svg
