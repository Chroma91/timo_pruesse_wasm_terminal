use crate::commands::timo_help::*;
use crate::commands::timo_version::*;
use crate::commands::TerminalCommand;

pub const TIMO_COMMAND_NAME: &str = "timo";

pub struct TimoCommand {
    pub args: Vec<String>,
}

impl TerminalCommand for TimoCommand {
    fn run(&self) -> Result<String, String> {
        let help_command = &TIMO_HELP_COMMAND_FLAG.to_string();
        let subcommand = self.args.get(0).or(Some(help_command)).unwrap();

        if subcommand == help_command {
            return TimoHelpCommand {}.run();
        }

        return match subcommand.as_str() {
            TIMO_VERSION_COMMAND_FLAG => TimoVersionCommand {}.run(),
            _ => Err(format!("timo_wasm: command not found: {}", subcommand)),
        };
    }
}
