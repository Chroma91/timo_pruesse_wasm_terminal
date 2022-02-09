use crate::commands::TerminalCommand;

pub const WHOAMI_COMMAND_NAME: &str = "whoami";

pub struct WhoamiCommand {}

impl TerminalCommand for WhoamiCommand {
    fn run(&self) -> Result<String, String> {
        return Ok("timo_pruesse".to_string());
    }
}
