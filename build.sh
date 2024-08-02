#!/bin/bash

wasm-pack build --release --target nodejs --reference-types --weak-refs --features wasm --no-default-features
cp target/schemas/out/980faa7cf81d51eab3f26de770ae345d.xsdb pkg/xwasser-v050.xsdb.bin
cp package.tmp.json pkg/package.json