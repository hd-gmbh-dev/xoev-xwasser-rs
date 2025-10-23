#!/bin/bash
set -e
cargo clean
cargo test --no-default-features --features schema,trace,builder
wasm-pack build --release --target nodejs --reference-types --features wasm,builder --no-default-features
# wasm-pack build --release --target web --reference-types --features wasm --no-default-features
cp crates/codelists/public/V0_9_5/codelist.json pkg/codelist.json
cp target/schemas/out/*.xsdb pkg/xwasser-v095.xsdb.bin
#cp package.tmp.web.json pkg/package.json
cp package.tmp.json pkg/package.json
pnpm tsup --format esm,cjs