use crate::commands::TerminalCommand;
use typed_html::dom::DOMTree;
use typed_html::html;

pub const TIMO_GET_GITHUB_COMMAND_FLAG: &str = "get:github";

pub struct TimoGetGithubCommand {}

impl TerminalCommand for TimoGetGithubCommand {
    fn run(&self) -> Result<String, String> {
        let body: DOMTree<String> = html!(
        <div style="margin:0.5rem 0 1rem 2.5rem;display:flex;gap:1rem">
            <span>"GitHub:"</span>
            <a
                href="https://github.com/Chroma91"
                target="_blank"
                // rel="noopener noreferrer" // TODO: fix this
            >"Chroma91"</a
            >
        </div>
        );

        return Ok(body.to_string());
    }
}
