use crate::commands::TerminalCommand;

pub const TIMO_SYSTEM_COMMAND_FLAG: &str = "--system";

pub struct TimoSystemCommand {}

impl TerminalCommand for TimoSystemCommand {
    fn run(&self) -> Result<String, String> {
        return Ok("
            WSL 2

            |                |         |
            |----------------|---------|
            | Distributor ID | Debian  |
            | Description    | Pengwin |
            | Release        | 11      |
            | Codename       | bullseye |
        "
        .to_string());
    }
}
