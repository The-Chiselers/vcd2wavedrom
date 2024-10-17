#!/bin/sh

mkdir -p out

rm -f out/vcd.json out/wave.json
./venv/bin/python3 scripts/main.py --vcd_file ./examples/gpio_all.vcd > out/vcd.json
cargo run -- out/vcd.json examples/gpio_all.json | tee out/wave.json
wavedrom-cli --input out/wave.json --svg out/wave.svg
