#[cfg(test)]
extern crate timo_pruesse_wasm_terminal;

use timo_pruesse_wasm_terminal::commands::ls::LsCommand;
use timo_pruesse_wasm_terminal::commands::TerminalCommand;

#[test]
fn test_ls_command_project_dir() {
    let ls = LsCommand { args: vec![] };

    assert!(ls.run().unwrap().len() > 0);
}

#[test]
fn test_ls_command_home_dir() {
    let ls = LsCommand {
        args: vec!["~".to_string()],
    };

    assert_eq!(ls.run().unwrap(), "aboutme");
}
