use crate::commands::TerminalCommand;
use std::collections::HashMap;

pub const CAT_COMMAND_NAME: &str = "cat";

pub struct CatCommand {
    pub args: Vec<String>,
}

impl TerminalCommand for CatCommand {
    fn run(&self) -> Result<String, String> {
        let files: HashMap<&str, &str> = HashMap::from([(
            "~/aboutme",
            "
        Hi, my name is Timo 🙋‍♂️<br />
        I love teaching machines how to solve problems.
        <br />
    ",
        )]);

        if self.args.len() < 1 {
            return Err("timo_wasm: no file specified".to_string());
        }

        let file_name = &self.args[0].as_str();

        if !files.contains_key(file_name) {
            return Err(format!("timo_wasm: file {} not found", file_name));
        }

        return Ok(files.get(file_name).unwrap().to_string());
    }
}
