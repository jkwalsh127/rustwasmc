use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn add_numbers(n1: i32, n2:i32) -> i32{
    n1 + n2
}

#[wasm_bindgen]
pub fn square(n1: i32) -> i32{
    n1*n1
}