#!/bin/bash
cargo build --release --target wasm32-unknown-unknown
mkdir -p out
wasm-bindgen target/wasm32-unknown-unknown/release/wasm_lib.wasm --out-dir out --no-modules --no-typescript
cp out/* ../wasm_web_server/web/
