use crate::commands::TerminalCommand;

pub const TIMO_HELP_COMMAND_FLAG: &str = "--help";

pub struct TimoHelpCommand {}

impl TerminalCommand for TimoHelpCommand {
    fn get_description(&self) -> String {
        return "help desc".to_string();
    }

    fn run(&self) -> Result<String, String> {
        return Ok("help".to_string());
    }
}
