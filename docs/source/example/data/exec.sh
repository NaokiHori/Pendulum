#!/bin/bash

set -x
set -e

outdir=artifacts

mkdir ${outdir}

# implicit scheme
rm -f energy.dat
cargo run --release > implicit.log
python3 \
  docs/source/example/data/energy.py \
  energy.dat \
  ${outdir}/energy11.jpg \
  ${outdir}/energy12.jpg

# explicit scheme
rm -f energy.dat
cargo run --release --features=explicit > explicit.log
python3 \
  docs/source/example/data/energy.py \
  energy.dat \
  ${outdir}/energy21.jpg \
  ${outdir}/energy22.jpg
