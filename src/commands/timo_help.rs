use crate::commands::TerminalCommand;
use crate::utils::version::get_version;
use typed_html::dom::DOMTree;
use typed_html::html;
use typed_html::text;

pub const TIMO_HELP_COMMAND_FLAG: &str = "--help";

pub struct TimoHelpCommand {}

impl TerminalCommand for TimoHelpCommand {
    fn run(&self) -> Result<String, String> {
        let body: DOMTree<String> = html!(
        <div style="margin:0.5rem 0 1rem 2rem;display:flex;flex-direction:column;gap:1rem">
            <span style="font-weight:700">"Timo ("{ text!("{}", get_version()) }")"</span>
            <span>"Usage:"</span>
            <table style="margin-left:1rem">
                <tr><td>"--help"</td><td>"print this message"</td></tr>
                <tr><td>"--stack"</td><td>"print tech stack"</td></tr>
                <tr><td>"--version"</td><td>"print version"</td></tr>
                <tr><td style="padding-top:1rem">"get:contact"</td><td style="padding-top:1rem">"gets contact info"</td></tr>
                <tr><td>"get:github"</td><td>"gets GitHub name"</td></tr>
                <tr><td>"get:twitter"</td><td>"gets Twitter name"</td></tr>
                <tr><td>"get:instagram"</td><td>"gets Instagram name"</td></tr>
            </table>
            <span>"Examples:"</span>
            <table style="margin-left:1rem">
                <tr><td>"timo --version"</td><td>"print version"</td></tr>
                <tr><td>"timo get:contact"</td><td>"get contact info"</td></tr>
            </table>
            </div>
        );

        return Ok(body.to_string());
    }
}
