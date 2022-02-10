use crate::commands::TerminalCommand;
use std::collections::HashMap;

pub const LS_COMMAND_NAME: &str = "ls";

pub struct LsCommand {
    pub args: Vec<String>,
}

impl TerminalCommand for LsCommand {
    fn run(&self) -> Result<String, String> {
        if self.args.len() == 0 {
            return Ok("
            build         package.json       postcss.config.cjs  src     svelte.config.js     tsconfig.json
            node_modules  package-lock.json  README.md           static  tailwind.config.cjs
        "
            .to_string());
        }

        let directories: HashMap<&str, &str> = HashMap::from([("~", "aboutme")]);

        let directory_name = &self.args[0].as_str();

        if !directories.contains_key(directory_name) {
            return Err(format!("timo_wasm: directory {} not found", directory_name));
        }

        return Ok(directories.get(directory_name).unwrap().to_string());
    }
}
