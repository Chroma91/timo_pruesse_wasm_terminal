use crate::commands::TerminalCommand;
use typed_html::dom::DOMTree;
use typed_html::html;

pub const TIMO_GET_GITHUB_COMMAND_FLAG: &str = "get:github";

pub struct TimoGetGithubCommand {}

impl TerminalCommand for TimoGetGithubCommand {
    fn run(&self) -> Result<String, String> {
        let body: DOMTree<String> = html!(
        <div class="ml-10 mt-2 mb-4 flex gap-4">
            <span>"GitHub:"</span>
            <a
                href="https://github.com/Chroma91"
                class="hover:underline"
                target="_blank"
                // rel="noopener noreferrer" // TODO: fix this
            >"Chroma91"</a
            >
        </div>
        );

        return Ok(body.to_string());
    }
}
