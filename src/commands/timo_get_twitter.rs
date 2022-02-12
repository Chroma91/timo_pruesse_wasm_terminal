use crate::commands::TerminalCommand;
use typed_html::dom::DOMTree;
use typed_html::html;

pub const TIMO_GET_TWITTER_COMMAND_FLAG: &str = "get:twitter";

pub struct TimoGetTwitterCommand {}

impl TerminalCommand for TimoGetTwitterCommand {
    fn run(&self) -> Result<String, String> {
        let body: DOMTree<String> = html!(
            <div style="margin:0.5rem 0 1rem 2.5rem;display:flex;gap:1rem">
                <span>"Twitter:"</span>
                <a
                    href="https://twitter.com/TimoPruesse"
                    target="_blank"
                    // rel="noopener noreferrer" // TODO: fix this
                >"TimoPruesse"</a
                >
            </div>
        );

        return Ok(body.to_string());
    }
}
