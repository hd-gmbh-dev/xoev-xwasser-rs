#!/bin/bash
set -e
cargo clean
wasm-pack build --release --target web --reference-types --weak-refs --features wasm,builder --no-default-features
cp target/schemas/out/*.xsdb pkg/xwasser-v060.xsdb.bin
cp package.tmp.web.json pkg/package.json