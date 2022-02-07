use crate::commands::timo::TimoCommand;
use crate::commands::TerminalCommand;

pub fn parse_command(command: &str) -> Result<impl TerminalCommand, String> {
    let mut parts = command.split_whitespace();
    let name = parts.next().unwrap();
    let str_args = parts.collect::<Vec<_>>();
    let args = str_args.iter().map(|arg| arg.to_string()).collect();

    match name {
        "timo" => Ok(TimoCommand { args }),
        _ => Err(format!("Unknown command: {}", name)),
    }
}
