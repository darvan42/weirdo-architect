use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_two(x: isize) -> isize {
    x + 2
}
