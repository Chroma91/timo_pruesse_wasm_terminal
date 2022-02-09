use crate::commands::TerminalCommand;

pub const TIMO_GET_INSTAGRAM_COMMAND_FLAG: &str = "get:instagram";

pub struct TimoGetInstagramCommand {}

impl TerminalCommand for TimoGetInstagramCommand {
    fn run(&self) -> Result<String, String> {
        return Ok("
            |                    |             |
            |--------------------|-------------|
            | Instagram username | timopruesse |
        "
        .to_string());
    }
}
