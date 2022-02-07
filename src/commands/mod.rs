pub trait TerminalCommand {
    fn run(&self) -> Result<String, String>;
}

pub mod timo;
