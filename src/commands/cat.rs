use crate::commands::TerminalCommand;
use std::collections::HashMap;
use typed_html::dom::DOMTree;
use typed_html::html;

pub const CAT_COMMAND_NAME: &str = "cat";

pub struct CatCommand {
    pub args: Vec<String>,
}

impl TerminalCommand for CatCommand {
    fn run(&self) -> Result<String, String> {
        let aboutme: DOMTree<String> = html!(
            <div>
                <p>"Hi, my name is Timo 🙋‍♂️"</p>
                <p>"I love teaching machines how to solve problems."</p>
            </div>
        );

        let files: HashMap<&str, DOMTree<String>> = HashMap::from([("~/aboutme", aboutme)]);

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
