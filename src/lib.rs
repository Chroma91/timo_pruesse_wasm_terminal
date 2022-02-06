extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

pub mod command;
pub mod parser;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
