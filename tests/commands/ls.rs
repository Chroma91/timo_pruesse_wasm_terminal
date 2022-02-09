#[cfg(test)]
extern crate timo_pruesse_wasm_terminal;

use timo_pruesse_wasm_terminal::commands::ls::LsCommand;
use timo_pruesse_wasm_terminal::commands::TerminalCommand;

#[test]
fn test_ls_command() {
    let ls = LsCommand {};

    assert!(ls.run().unwrap().len() > 0);
}
