use crate::commands::TerminalCommand;
use typed_html::dom::DOMTree;
use typed_html::html;

pub const TIMO_GET_TWITTER_COMMAND_FLAG: &str = "get:twitter";

pub struct TimoGetTwitterCommand {}

impl TerminalCommand for TimoGetTwitterCommand {
    fn run(&self) -> Result<String, String> {
        let body: DOMTree<String> = html!(
            <div class="ml-10 mt-2 mb-4 flex gap-4">
                <span>"Twitter:"</span>
                <a
                    href="https://twitter.com/TimoPruesse"
                    class="hover:underline"
                    target="_blank"
                    // rel="noopener noreferrer" // TODO: fix this
                >"TimoPruesse"</a
                >
            </div>
        );

        return Ok(body.to_string());
    }
}
