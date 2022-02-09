pub trait TerminalCommand {
    fn run(&self) -> Result<String, String>;
}

pub mod cat;
pub mod ls;
pub mod timo;
pub mod timo_help;
pub mod timo_version;
pub mod whoami;
