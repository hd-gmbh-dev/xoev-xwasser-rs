#!/bin/bash
# cargo clean
# cargo test --features schema,trace
# wasm-pack build --release --target nodejs --reference-types --weak-refs --features wasm --no-default-features
wasm-pack build --release --target nodejs --reference-types --weak-refs --features wasm --no-default-features
cp target/schemas/out/*.xsdb pkg/xwasser-v052.xsdb.bin
cp package.tmp.json pkg/package.json
