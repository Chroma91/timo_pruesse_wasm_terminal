use crate::commands::TerminalCommand;

pub struct TimoCommand {
    pub args: Vec<String>,
}

fn help() -> String {
    return "help".to_string();
}

fn version() -> String {
    return "v1.0.0".to_string();
}

impl TerminalCommand for TimoCommand {
    fn run(&self) -> Result<String, String> {
        let help_command = &"--help".to_string();
        let subcommand = self.args.get(0).or(Some(help_command)).unwrap();

        if subcommand == help_command {
            return Ok(help());
        }

        return match subcommand.as_str() {
            "--version" => Ok(version()),
            _ => Err(format!("timo_wasm: command not found: {}", subcommand)),
        };
    }
}
