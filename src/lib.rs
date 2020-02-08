extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    pub fn simple_greet(s: &str);
}

#[wasm_bindgen(module = "/src/hello.js")]
extern "C" {
    pub fn reply_greet(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let s = format!("Hello, {}!", name);
    log(&format!("call {}", s));
    reply_greet(&s);
    simple_greet(&s);
}
