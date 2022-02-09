use crate::commands::TerminalCommand;

pub const WHOAMI_COMMAND_NAME: &str = "whoami";

pub struct WhoamiCommand {}

impl TerminalCommand for WhoamiCommand {
    fn get_description(&self) -> String {
        return "gets the current user name".to_string();
    }

    fn run(&self) -> Result<String, String> {
        return Ok("timo_pruesse".to_string());
    }
}
