#!/bin/bash
set -e
# cargo clean
wasm-pack build --release --target web --reference-types --features wasm,builder --no-default-features
cp crates/codelists/public/V1_0_0/codelist.json pkg/codelist.json
cp target/schemas/out/*.xsdb pkg/xwasser-v100.xsdb.bin
cp package.tmp.web.json pkg/package.json
pnpm tsup --format esm,cjs