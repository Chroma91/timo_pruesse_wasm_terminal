#[cfg(test)]
extern crate timo_pruesse_wasm_terminal;

use timo_pruesse_wasm_terminal::commands::whoami::WhoamiCommand;
use timo_pruesse_wasm_terminal::commands::TerminalCommand;

#[test]
fn test_whoami_command() {
    let whoami = WhoamiCommand {};

    assert!(whoami.run().is_ok());
}
