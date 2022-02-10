use crate::commands::TerminalCommand;
use crate::utils::version::get_version;

pub const TIMO_HELP_COMMAND_FLAG: &str = "--help";

pub struct TimoHelpCommand {}

impl TerminalCommand for TimoHelpCommand {
    fn run(&self) -> Result<String, String> {
        return Ok(format!(
            "
        <br />
        Timo ({version})
        <br />
        Usage:
        <br />
        |                |      |
        |----------------|------|
        | timo --help    | print this help message |
        | timo --stack   | print tech stack |
        | timo --version | print version |
        |                |      |
        | timo get:contact | gets contact info |
        | timo get:github | gets GitHub name |
        | timo get:twitter | gets Twitter name |
        | timo get:instagram | gets Instagram name |
        <br />
        ",
            version = get_version()
        ));
    }
}
