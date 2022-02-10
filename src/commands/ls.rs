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
            <div class="ml-8 mt-2 mb-4">
                <div class="grid grid-flow-col grid-cols-auto-fit grid-rows-6 md:grid-rows-4 lg:grid-rows-3 xl:grid-rows-2 gap-x-4">
                    <span class="text-turquoise whitespace-nowrap">"build"</span>
                    <span class="whitespace-nowrap">"package.json"</span>
                    <span class="whitespace-nowrap">"postcss.config.cjs"</span>
                    <span class="text-turquoise whitespace-nowrap">"src"</span>
                    <span class="whitespace-nowrap">"tsconfig.json"</span>
                    <span class="text-turquoise whitespace-nowrap">"node_modules"</span>
                    <span class="whitespace-nowrap">"package-lock.json"</span>
                    <span class="whitespace-nowrap">"README.md"</span>
                    <span class="text-turquoise whitespace-nowrap">"static"</span>
                    <span class="whitespace-nowrap">"tailwind.config.cjs"</span>
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
