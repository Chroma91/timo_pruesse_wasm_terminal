use crate::commands::TerminalCommand;
use typed_html::dom::DOMTree;
use typed_html::html;

pub const TIMO_SYSTEM_COMMAND_FLAG: &str = "--system";

pub struct TimoSystemCommand {}

impl TerminalCommand for TimoSystemCommand {
    fn run(&self) -> Result<String, String> {
        let body: DOMTree<String> = html!(
        <div style="margin:0.5rem 0 1rem 2rem">
            <table class="info-table">
                <tr><th colspan="2">"WSL 2"</th></tr>
                <tr><td style="padding-top:0.5rem">"Distributor ID"</td><td style="padding-top:0.5rem">"Ubuntu"</td></tr>
                <tr><td>"Description"</td><td>"Ubuntu 20.04.4 LTS"</td></tr>
                <tr><td>"Release"</td><td>"20.04"</td></tr>
                <tr><td>"Codename"</td><td>"focal"</td></tr>
            </table>
        </div>
        );

        return Ok(body.to_string());
    }
}
