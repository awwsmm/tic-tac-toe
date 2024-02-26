#!/usr/bin/env bash

# build
cargo build --release --target wasm32-unknown-unknown

# bind
wasm-bindgen --out-dir target --target web target/wasm32-unknown-unknown/release/tic-tac-toe.wasm