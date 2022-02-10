#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use timo_pruesse_wasm_terminal::run_command;
use timo_pruesse_wasm_terminal::utils::version::get_version;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_run_command() {
    assert_eq!(run_command(&"timo --version"), get_version());
}
