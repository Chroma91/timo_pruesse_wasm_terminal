#[cfg(test)]
extern crate timo_pruesse_wasm_terminal;

use crate::timo_pruesse_wasm_terminal::command::TerminalCommand;
use timo_pruesse_wasm_terminal::parser::*;

#[test]
fn test_parse_command() {
    let add_command = parse_command("add 1 2").unwrap();

    assert_eq!(add_command.run().unwrap(), "3");
}

#[test]
fn test_parse_invalid_command() {
    assert!(parse_command("failing command").is_err());
}
