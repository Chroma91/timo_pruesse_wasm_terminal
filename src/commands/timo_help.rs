use crate::commands::TerminalCommand;
use crate::utils::version::get_version;

pub const TIMO_HELP_COMMAND_FLAG: &str = "--help";

pub struct TimoHelpCommand {}

impl TerminalCommand for TimoHelpCommand {
    fn run(&self) -> Result<String, String> {
        return Ok(format!(
            "

        Timo ({version})

        Usage:

        --help             print this help message
        --stack            print tech stack
        --version          print version

        get:contact        gets contact info
        get:github         gets GitHub name
        get:twitter        gets Twitter name
        get:instagram      gets Instagram name

        Examples:
            timo --version      print version
            timo get:contact    get contact info

        ",
            version = get_version()
        ));
    }
}
