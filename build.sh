#/usr/bin/env sh

cargo rustc --lib --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir web target/wasm32-unknown-unknown/release/m6809.wasm
wasm-gc web/m6809_bg.wasm
