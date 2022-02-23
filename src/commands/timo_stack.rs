use crate::commands::TerminalCommand;
use typed_html::dom::DOMTree;
use typed_html::html;

pub const TIMO_STACK_COMMAND_FLAG: &str = "--stack";

pub struct TimoStackCommand {}

impl TerminalCommand for TimoStackCommand {
    fn run(&self) -> Result<String, String> {
        let body: DOMTree<String> = html!(
        <div style="margin:0.5rem 0 1rem 2rem;display:flex;flex-direction:column;gap:0.5rem">
        <div class="info-row">
            <span>"Languages:"</span>
            <div class="info-box">
                <div>
                    <span style="font-weight:700;font-size:0.875rem;line-height:1.25rem">"Daily Drivers"</span>
                    <span>"JavaScript/TypeScript, PHP, HTML, CSS"</span>
                </div>
                <div>
                    <span style="font-weight:700;font-size:0.875rem;line-height:1.25rem">"Experiments"</span>
                    <span>"Rust, Golang, Dart, Python"</span>
                </div>
            </div>
        </div>
        <div class="info-row">
            <span>"Frameworks:"</span>
            <div class="info-box">
                <div>
                    <span style="font-weight:700;font-size:0.875rem;line-height:1.25rem">"Daily Drivers"</span>
                    <span>"Laravel, Svelte/SvelteKit, React"</span>
                </div>
                <div>
                    <span style="font-weight:700;font-size:0.875rem;line-height:1.25rem">"Experiments"</span>
                    <span>"Flutter, Vue"</span>
                </div>
            </div>
        </div>
        <div class="info-row">
            <span>"Testing:"</span>
            <div class="info-box">
                <div>
                    <span style="font-weight:700;font-size:0.875rem;line-height:1.25rem">"Daily Drivers"</span>
                    <span>"Jest, PHPUnit, React Testing Library"</span>
                </div>
                <div>
                    <span style="font-weight:700;font-size:0.875rem;line-height:1.25rem">"Experiments"</span>
                    <span>"Playwright"</span>
                </div>
            </div>
        </div>
        <div class="info-row">
            <span>"Data Stores:"</span>
            <div class="info-box">
                <div>
                    <span style="font-weight:700;font-size:0.875rem;line-height:1.25rem">"Daily Drivers"</span>
                    <span>"AWS RDS, MySQL, Redis"</span>
                </div>
                <div>
                    <span style="font-weight:700;font-size:0.875rem;line-height:1.25rem">"Experiments"</span>
                    <span>"SQLite, MongoDB, PostgreSQL"</span>
                </div>
            </div>
        </div>
        <div class="info-row">
            <span>"Cloud/Deployment:"</span>
            <div class="info-box">
                <div>
                    <span style="font-weight:700;font-size:0.875rem;line-height:1.25rem">"Daily Drivers"</span>
                    <span>"AWS, DigitalOcean"</span>
                </div>
                <div>
                    <span style="font-weight:700;font-size:0.875rem;line-height:1.25rem">"Experiments"</span>
                    <span>"Firebase"</span>
                </div>
            </div>
        </div>
        <div class="info-row">
            <span>"AI/Data Science:"</span>
            <div class="info-box">
                <div>
                    <span style="font-weight:700;font-size:0.875rem;line-height:1.25rem">"Daily Drivers"</span>
                    <span>"-"</span>
                </div>
                <div>
                    <span style="font-weight:700;font-size:0.875rem;line-height:1.25rem">"Experiments"</span>
                    <span>"Tensorflow"</span>
                </div>
            </div>
        </div>
        </div>
           );

        return Ok(body.to_string());
    }
}
