cargo build --target wasm32-unknown-unknown
wasm-bindgen target\wasm32-unknown-unknown\debug\hello_wasm.wasm --out-dir .\pkg --target web
