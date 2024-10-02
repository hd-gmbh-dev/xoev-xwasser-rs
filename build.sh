#!/bin/bash
set -e
# cargo clean
# cargo test --no-default-features --features schema,trace,builder
wasm-pack build --release --target nodejs --reference-types --weak-refs --features wasm,builder --no-default-features
# wasm-pack build --release --target web --reference-types --weak-refs --features wasm --no-default-features
cp target/schemas/out/*.xsdb pkg/xwasser-v060.xsdb.bin
#cp package.tmp.web.json pkg/package.json
cp package.tmp.json pkg/package.json
pnpm tsup