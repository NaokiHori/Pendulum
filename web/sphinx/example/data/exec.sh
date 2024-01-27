#!/bin/bash

set -x
set -e

mkdir result

# implicit scheme
rm -f energy.dat
cargo run --release
python3 \
  web/sphinx/example/data/energy.py \
  energy.dat \
  result/energy11.jpg \
  result/energy12.jpg
cp energy.dat result/energy1.dat

# explicit scheme
rm -f energy.dat
cargo run --release --features=explicit
python3 \
  web/sphinx/example/data/energy.py \
  energy.dat \
  result/energy21.jpg \
  result/energy22.jpg
cp energy.dat result/energy2.dat
