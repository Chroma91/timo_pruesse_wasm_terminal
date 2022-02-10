use crate::commands::TerminalCommand;

pub const TIMO_GET_TWITTER_COMMAND_FLAG: &str = "get:twitter";

pub struct TimoGetTwitterCommand {}

impl TerminalCommand for TimoGetTwitterCommand {
    fn run(&self) -> Result<String, String> {
        return Ok("

            Twitter username    [TimoPruesse](https://twitter.com/TimoPruesse)
        "
        .to_string());
    }
}
