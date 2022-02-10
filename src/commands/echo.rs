use crate::commands::TerminalCommand;

pub const ECHO_COMMAND_NAME: &str = "echo";

pub struct EchoCommand {
    pub args: Vec<String>,
}

impl TerminalCommand for EchoCommand {
    fn run(&self) -> Result<String, String> {
        let mut args = self.args.clone();
        let arg_count = args.len();
        if arg_count > 0 {
            args[0] = args[0].trim_start_matches('"').to_string();
            args[arg_count - 1] = args[arg_count - 1].trim_end_matches('"').to_string();
        }

        return Ok(args.join(" "));
    }
}
