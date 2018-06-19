cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/wasmparse_bridge.wasm \
--out-dir pkg