#!/bin/bash
set -e
# cargo clean
wasm-pack build --release --target web --reference-types --features wasm,builder --no-default-features
cp crates/codelists/public/V0_7_2/codelist.json pkg/codelist.json
cp target/schemas/out/*.xsdb pkg/xwasser-v070.xsdb.bin
cp package.tmp.web.json pkg/package.json
pnpm tsup --format esm,cjs