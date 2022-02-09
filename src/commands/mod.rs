pub trait TerminalCommand {
    fn get_description(&self) -> String;
    fn run(&self) -> Result<String, String>;
}

pub mod timo;
pub mod timo_help;
pub mod timo_version;
pub mod whoami;
