use crate::commands::TerminalCommand;

pub const TIMO_GET_GITHUB_COMMAND_FLAG: &str = "get:github";

pub struct TimoGetGithubCommand {}

impl TerminalCommand for TimoGetGithubCommand {
    fn run(&self) -> Result<String, String> {
        return Ok("
            |                 |          |
            |-----------------|----------|
            | GitHub username | Chroma91 |
        "
        .to_string());
    }
}
