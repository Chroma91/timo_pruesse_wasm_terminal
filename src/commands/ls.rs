use crate::commands::TerminalCommand;
use std::collections::HashMap;
use typed_html::dom::DOMTree;
use typed_html::html;

pub const LS_COMMAND_NAME: &str = "ls";

pub struct LsCommand {
    pub args: Vec<String>,
}

impl TerminalCommand for LsCommand {
    fn run(&self) -> Result<String, String> {
        if self.args.len() == 0 {
            let body: DOMTree<String> = html!(
            <div style="margin:0.5rem 0 1rem 2rem">
                <div class="file-list">
                    <span class="highlight-file">"build"</span>
                    <span>"package.json"</span>
                    <span>"postcss.config.cjs"</span>
                    <span class="highlight-file">"src"</span>
                    <span>"tsconfig.json"</span>
                    <span class="highlight-file">"node_modules"</span>
                    <span>"package-lock.json"</span>
                    <span>"README.md"</span>
                    <span class="highlight-file">"static"</span>
                    <span>"tailwind.config.cjs"</span>
                </div>
            </div>
            );

            return Ok(body.to_string());
        }

        let directories: HashMap<&str, &str> = HashMap::from([("~", "aboutme")]);

        let directory_name = &self.args[0].as_str();

        if !directories.contains_key(directory_name) {
            return Err(format!("timo_wasm: directory {} not found", directory_name));
        }

        return Ok(directories.get(directory_name).unwrap().to_string());
    }
}
