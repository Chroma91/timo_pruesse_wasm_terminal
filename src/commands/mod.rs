pub trait TerminalCommand {
    fn run(&self) -> Result<String, String>;
}

pub mod cat;
pub mod echo;
pub mod ls;
pub mod timo;
pub mod timo_get_contact;
pub mod timo_get_github;
pub mod timo_get_instagram;
pub mod timo_get_twitter;
pub mod timo_help;
pub mod timo_stack;
pub mod timo_system;
pub mod timo_version;
pub mod whoami;
