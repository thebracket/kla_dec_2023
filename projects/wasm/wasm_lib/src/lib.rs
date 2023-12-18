use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn hello_js() {
    log("Hello from Rust!");
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn greet(s: String) -> String {
    format!("Hello {s}")
}

#[wasm_bindgen]
pub fn sum(arr: &[i32]) -> i32 {
    arr.iter().sum()
}