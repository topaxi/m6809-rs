#/usr/bin/env sh

cargo rustc --lib --release --target wasm32-unknown-unknown
wasm-bindgen --browser --out-dir web target/wasm32-unknown-unknown/release/m6809.wasm
