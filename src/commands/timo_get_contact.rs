use crate::commands::TerminalCommand;

pub const TIMO_GET_CONTACT_COMMAND_FLAG: &str = "get:contact";

pub struct TimoGetContactCommand {}

impl TerminalCommand for TimoGetContactCommand {
    fn run(&self) -> Result<String, String> {
        return Ok("
            |              |                        |
            |--------------|------------------------|
            | email        | hi@timo-pruesse.de     |
            | @niceoutside | https://www.no.studio/ |
        "
        .to_string());
    }
}
