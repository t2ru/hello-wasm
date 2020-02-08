extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen(module = "/src/hello.js")]
extern "C" {
    pub fn reply_greet(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let s = format!("Hello, {}!", name);
    alert(&s);
    reply_greet(&s);
}
