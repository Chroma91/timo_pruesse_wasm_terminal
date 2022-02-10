use crate::commands::TerminalCommand;
use typed_html::dom::DOMTree;
use typed_html::html;

pub const TIMO_SYSTEM_COMMAND_FLAG: &str = "--system";

pub struct TimoSystemCommand {}

impl TerminalCommand for TimoSystemCommand {
    fn run(&self) -> Result<String, String> {
        let body: DOMTree<String> = html!(
        <div class="ml-8 mt-2 mb-4">
            <table class="md:w-1/2 lg:w-1/3 w-full">
                <tr><th colspan="2" class="text-left border-b border-iceblue/30">"WSL 2"</th></tr>
                <tr><td class="pt-2">"Distributor ID"</td><td class="pt-2">"Debian"</td></tr>
                <tr><td>"Description"</td><td>"Pengwin"</td></tr>
                <tr><td>"Release"</td><td>"11"</td></tr>
                <tr><td>"Codename"</td><td>"bullseye"</td></tr>
            </table>
        </div>
        );

        return Ok(body.to_string());
    }
}
