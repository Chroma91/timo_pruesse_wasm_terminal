use crate::commands::TerminalCommand;
use typed_html::dom::DOMTree;
use typed_html::html;

pub const TIMO_GET_INSTAGRAM_COMMAND_FLAG: &str = "get:instagram";

pub struct TimoGetInstagramCommand {}

impl TerminalCommand for TimoGetInstagramCommand {
    fn run(&self) -> Result<String, String> {
        let body: DOMTree<String> = html!(
            <div style="margin:0.5rem 0 1rem 2.5rem;display:flex;gap:1rem">
                <span>"Instagram:"</span>
                <a
                    href="https://www.instagram.com/timopruesse/"
                    target="_blank"
                    // rel="noopener noreferrer" // TODO: fix this
                >"timopruesse"</a
                >
            </div>
        );

        return Ok(body.to_string());
    }
}
