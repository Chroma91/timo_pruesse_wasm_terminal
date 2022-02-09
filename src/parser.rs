use crate::commands::timo::*;
use crate::commands::whoami::*;
use crate::commands::TerminalCommand;

pub fn parse_command(command: &str) -> Result<Box<dyn TerminalCommand>, String> {
    let mut parts = command.split_whitespace();
    let name = parts.next().unwrap();
    let str_args = parts.collect::<Vec<_>>();
    let args = str_args.iter().map(|arg| arg.to_string()).collect();

    match name {
        TIMO_COMMAND_NAME => Ok(Box::new(TimoCommand { args })),
        WHOAMI_COMMAND_NAME => Ok(Box::new(WhoamiCommand {})),
        _ => Err(format!("Unknown command: {}", name)),
    }
}
