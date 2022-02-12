use crate::commands::TerminalCommand;
use typed_html::dom::DOMTree;
use typed_html::html;

pub const TIMO_GET_CONTACT_COMMAND_FLAG: &str = "get:contact";

pub struct TimoGetContactCommand {}

impl TerminalCommand for TimoGetContactCommand {
    fn run(&self) -> Result<String, String> {
        let body: DOMTree<String> = html!(
        <div style="margin:0.5rem 0 1rem 2.75rem;display:flex;gap:1rem">
            <div style="display:flex;flex-direction:column">
                <span>"email:"</span>
                <span>"@niceoutside:"</span>
            </div>
            <div style="display:flex;flex-direction:column">
                <a href="mailto:hi@timo-pruesse.de">"hi@timo-pruesse.de"</a>
                <a
                    href="https://www.no.studio/"
                    target="_blank"
                    // rel="noopener noreferrer" // TODO: fix this
                >"no.studio"</a>
            </div>
        </div>
        );

        return Ok(body.to_string());
    }
}
