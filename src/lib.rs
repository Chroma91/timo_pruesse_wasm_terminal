extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

pub mod commands;
pub mod parser;

use commands::TerminalCommand;

#[wasm_bindgen]
pub fn run_command(command_str: &str) -> String {
    let command = parser::parse_command(command_str);

    match command {
        Ok(command) => command.run().unwrap(),
        Err(error) => error,
    }
}
