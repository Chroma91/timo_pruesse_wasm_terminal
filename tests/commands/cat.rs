#[cfg(test)]
extern crate timo_pruesse_wasm_terminal;

use timo_pruesse_wasm_terminal::commands::cat::CatCommand;
use timo_pruesse_wasm_terminal::commands::TerminalCommand;

#[test]
fn test_cat_command_missing_filename() {
    let cat = CatCommand { args: vec![] };

    assert_eq!(cat.run().unwrap_err(), "timo_wasm: no file specified");
}

#[test]
fn test_cat_command_unknown_file() {
    let cat = CatCommand {
        args: vec!["invalid_file".to_string()],
    };

    assert_eq!(
        cat.run().unwrap_err(),
        "timo_wasm: file invalid_file not found"
    );
}

#[test]
fn test_cat_command() {
    let cat = CatCommand {
        args: vec!["aboutme".to_string()],
    };

    assert!(cat.run().unwrap().len() > 0);
}
