use crate::command::AddCommand;
use crate::command::*;

pub fn parse_command(command: &str) -> Result<impl TerminalCommand, String> {
    let mut parts = command.split_whitespace();
    let name = parts.next().unwrap();
    let str_args = parts.collect::<Vec<_>>();
    let args = str_args.iter().map(|arg| arg.to_string()).collect();

    match name {
        "add" => Ok(AddCommand { args }),
        _ => Err(format!("Unknown command: {}", name)),
    }
}
