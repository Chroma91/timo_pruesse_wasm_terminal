use crate::commands::TerminalCommand;
use typed_html::dom::DOMTree;
use typed_html::html;

pub const TIMO_GET_CONTACT_COMMAND_FLAG: &str = "get:contact";

pub struct TimoGetContactCommand {}

impl TerminalCommand for TimoGetContactCommand {
    fn run(&self) -> Result<String, String> {
        let body: DOMTree<String> = html!(
        <div class="ml-11 mt-2 mb-4 flex gap-4">
            <div class="flex flex-col">
                <span>"email:"</span>
                <span>"@niceoutside:"</span>
            </div>
            <div class="flex flex-col">
                <a href="mailto:hi@timo-pruesse.de" class="hover:underline">"hi@timo-pruesse.de"</a>
                <a
                    href="https://www.no.studio/"
                    class="hover:underline"
                    target="_blank"
                    // rel="noopener noreferrer" // TODO: fix this
                >"no.studio"</a>
            </div>
        </div>
        );

        return Ok(body.to_string());
    }
}
