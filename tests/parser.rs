#[cfg(test)]
extern crate timo_pruesse_wasm_terminal;

use crate::timo_pruesse_wasm_terminal::commands::TerminalCommand;
use timo_pruesse_wasm_terminal::parser::*;

#[test]
fn test_parse_command() {
    let timo_command = parse_command("timo --version").unwrap();

    assert_eq!(timo_command.run().unwrap(), "v1.0.0");
}

#[test]
fn test_parse_invalid_command() {
    assert!(parse_command("failing command").is_err());
}
