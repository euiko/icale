mod utils;

use wasm_bindgen::prelude::*;
use icale_core;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let now = icale_core::now_utc();
    let str : std::string::String = format!("Hello, wasm-rs! Now is {}", now.timestamp());
    alert(&str);
}
