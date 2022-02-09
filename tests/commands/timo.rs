#[cfg(test)]
extern crate timo_pruesse_wasm_terminal;

use timo_pruesse_wasm_terminal::commands::timo::TimoCommand;
use timo_pruesse_wasm_terminal::commands::TerminalCommand;
use timo_pruesse_wasm_terminal::utils::version::get_version;

#[test]
fn test_no_command() {
    let timo = TimoCommand { args: vec![] };

    assert!(timo.run().unwrap().len() > 0);
}

#[test]
fn test_version_command() {
    let timo = TimoCommand {
        args: vec!["--version".to_string()],
    };

    assert_eq!(timo.run().unwrap(), get_version());
}

#[test]
fn test_invalid_command() {
    let timo = TimoCommand {
        args: vec!["invalid".to_string()],
    };

    assert_eq!(
        timo.run().unwrap_err(),
        "timo_wasm: command not found: invalid"
    );
}
