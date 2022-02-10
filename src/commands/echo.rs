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
            let end_index = arg_count - 1;

            args[0] = args[0]
                .strip_prefix('"')
                .or(Some(&args[0]))
                .unwrap()
                .to_string();

            args[end_index] = args[end_index]
                .strip_suffix('"')
                .or(Some(&args[end_index]))
                .unwrap()
                .to_string();
        }

        return Ok(args.join(" "));
    }
}
