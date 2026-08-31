#!/bin/bash
set -e
# cargo clean
wasm-pack build --release --target web --reference-types --features wasm,builder --no-default-features
cp crates/codelists/public/V1_0_1/codelist.json pkg/codelist.json
cp target/schemas/out/*.xsdb pkg/xwasser-v101.xsdb.bin
cp package.tmp.web.json pkg/package.json
cp README.npm.md pkg/README.md
pnpm tsup --format esm,cjs