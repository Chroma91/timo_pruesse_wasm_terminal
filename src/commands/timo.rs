use crate::commands::timo_get_contact::*;
use crate::commands::timo_get_github::*;
use crate::commands::timo_get_instagram::*;
use crate::commands::timo_get_twitter::*;
use crate::commands::timo_help::*;
use crate::commands::timo_stack::*;
use crate::commands::timo_system::*;
use crate::commands::timo_version::*;
use crate::commands::TerminalCommand;

pub const TIMO_COMMAND_NAME: &str = "timo";

pub struct TimoCommand {
    pub args: Vec<String>,
}

impl TerminalCommand for TimoCommand {
    fn run(&self) -> Result<String, String> {
        let help_command = &TIMO_HELP_COMMAND_FLAG.to_string();
        let subcommand = self.args.get(0).or(Some(help_command)).unwrap();

        return match subcommand.as_str() {
            TIMO_HELP_COMMAND_FLAG => TimoHelpCommand {}.run(),
            TIMO_STACK_COMMAND_FLAG => TimoStackCommand {}.run(),
            TIMO_SYSTEM_COMMAND_FLAG => TimoSystemCommand {}.run(),
            TIMO_VERSION_COMMAND_FLAG => TimoVersionCommand {}.run(),
            TIMO_GET_CONTACT_COMMAND_FLAG => TimoGetContactCommand {}.run(),
            TIMO_GET_TWITTER_COMMAND_FLAG => TimoGetTwitterCommand {}.run(),
            TIMO_GET_GITHUB_COMMAND_FLAG => TimoGetGithubCommand {}.run(),
            TIMO_GET_INSTAGRAM_COMMAND_FLAG => TimoGetInstagramCommand {}.run(),
            _ => Err(format!(
                "timo_wasm: {}: command not found: {}",
                TIMO_COMMAND_NAME, subcommand
            )),
        };
    }
}
