use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn hello_screeps() {
    console::log_1(&JsValue::from("Hello from the other side! (Rust is here)"))
}
