use crate::commands::TerminalCommand;

pub const LS_COMMAND_NAME: &str = "ls";

pub struct LsCommand {}

impl TerminalCommand for LsCommand {
    fn run(&self) -> Result<String, String> {
        return Ok("
        |                |      |
        |----------------|------|
        | build | package-lock.json | src | tailwind.config.cjs
        | node_modules | postcss.config.cjs | static | tsconfig.json
        | package.json | README.md | svelte.config.js
        "
        .to_string());
    }
}
