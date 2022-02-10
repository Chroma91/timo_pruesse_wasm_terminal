#[cfg(test)]
extern crate timo_pruesse_wasm_terminal;

use timo_pruesse_wasm_terminal::commands::echo::EchoCommand;
use timo_pruesse_wasm_terminal::commands::TerminalCommand;

#[test]
fn test_echo_command() {
    let echo = EchoCommand {
        args: vec!["hello".to_string(), "world".to_string()],
    };

    assert_eq!(echo.run().unwrap(), "hello world");
}

#[test]
fn test_echo_command_empty() {
    let echo = EchoCommand { args: vec![] };

    assert_eq!(echo.run().unwrap(), "");
}
