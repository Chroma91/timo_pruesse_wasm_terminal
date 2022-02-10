use crate::commands::TerminalCommand;
use typed_html::dom::DOMTree;
use typed_html::html;

pub const TIMO_GET_INSTAGRAM_COMMAND_FLAG: &str = "get:instagram";

pub struct TimoGetInstagramCommand {}

impl TerminalCommand for TimoGetInstagramCommand {
    fn run(&self) -> Result<String, String> {
        let body: DOMTree<String> = html!(
            <div class="ml-10 mt-2 mb-4 flex gap-4">
                <span>"Instagram:"</span>
                <a
                    href="https://www.instagram.com/timopruesse/"
                    class="hover:underline"
                    target="_blank"
                    // rel="noopener noreferrer" // TODO: fix this
                >"timopruesse"</a
                >
            </div>
        );

        return Ok(body.to_string());
    }
}
