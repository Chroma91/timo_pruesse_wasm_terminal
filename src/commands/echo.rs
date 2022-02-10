use crate::commands::TerminalCommand;

pub const ECHO_COMMAND_NAME: &str = "echo";

pub struct EchoCommand {
    pub args: Vec<String>,
}

impl TerminalCommand for EchoCommand {
    fn run(&self) -> Result<String, String> {
        return Ok(self.args.join(" "));
    }
}
