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

use serde::Serialize;

#[derive(Serialize)]
#[wasm_bindgen]
pub struct Person {
    name: String,
    age: u8,
}

#[wasm_bindgen]
impl Person {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    pub fn greet(&self) -> String {
        format!("Hello, my name is {} and I am {} years old", self.name, self.age)
    }

    pub fn set_age(&mut self, age: u8) {
        self.age = age;
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }
}

#[wasm_bindgen]
pub fn serialize_person(person: &Person) -> String {
    serde_json::to_string(person).unwrap()
}

use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub async fn fetch_hello_json() -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("/json");

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Send the JSON response back to JS.
    Ok(json)
}