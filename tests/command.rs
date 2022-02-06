#[cfg(test)]
extern crate timo_pruesse_wasm_terminal;

use timo_pruesse_wasm_terminal::command::*;

#[test]
fn test_add_command() {
    let add = AddCommand {
        args: vec!["1".to_string(), "2".to_string()],
    };
    let result = add.run();

    assert_eq!(result.unwrap(), "3");
}

#[test]
fn test_invalid_number_of_args() {
    let add = AddCommand {
        args: vec!["5".to_string()],
    };

    assert!(add.run().is_err());
}
