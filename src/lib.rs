// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;

// Our Add function
// wasm-pack requires "exported" functions
// to include #[wasm_bindgen]
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn return_string() -> String {
    "hello".into()
}

#[wasm_bindgen]
pub fn hello(name: &str) -> JsValue {
    JsValue::from_str(&format!("Hello from rust, {}!", name))
}

#[wasm_bindgen]
pub fn hello_str() -> String {
    // return "hello".to_string();
    // format!("Hello {}", "ddd")
    let mut tr = String::from("");
    for i in 0..10000 {
        tr.push_str("<tr>");
        tr.push_str(&format!("<td>{}</td>", i+1));
        for _ in 0..30{
            tr.push_str("<td>test</td>");
        }
        tr.push_str("</tr>");
    }
    return tr.to_string();
}