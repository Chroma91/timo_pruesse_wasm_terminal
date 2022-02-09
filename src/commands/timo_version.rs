use crate::commands::TerminalCommand;
use crate::utils::version::get_version;

pub const TIMO_VERSION_COMMAND_FLAG: &str = "--version";

pub struct TimoVersionCommand {}

impl TerminalCommand for TimoVersionCommand {
    fn get_description(&self) -> String {
        return "prints the timo version".to_string();
    }

    fn run(&self) -> Result<String, String> {
        return Ok(get_version());
    }
}
