#[cfg(test)]
extern crate timo_pruesse_wasm_terminal;

use crate::timo_pruesse_wasm_terminal::utils::version::get_version;
use timo_pruesse_wasm_terminal::parser::*;

#[test]
fn test_parse_command() {
    let timo_command = parse_command("timo --version").unwrap();

    assert_eq!(timo_command.run().unwrap(), get_version());
}

#[test]
fn test_parse_invalid_command() {
    assert!(parse_command("failing command").is_err());
}
