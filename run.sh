#!/bin/sh

mkdir -p out

./venv/bin/python3 scripts/main.py --vcd_file ./examples/gpio_all.vcd > out/vcd.json
cargo run --release -- out/vcd.json examples/gpio_all.json  > out/wave.json
