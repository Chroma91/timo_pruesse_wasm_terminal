use crate::commands::TerminalCommand;
use typed_html::dom::DOMTree;
use typed_html::html;

pub const TIMO_STACK_COMMAND_FLAG: &str = "--stack";

pub struct TimoStackCommand {}

impl TerminalCommand for TimoStackCommand {
    fn run(&self) -> Result<String, String> {
        let body: DOMTree<String> = html!(
        <div class="ml-8 mt-2 mb-4 flex flex-col gap-1">
        <div class="flex gap-8">
            <span class="w-1/4">"Languages:"</span>
            <div
                class="flex-1 border border-iceblue/10 border-separate border-dashed flex pt-1 px-2 mb-2 text-base"
            >
                <div class="flex-1 flex flex-col pr-2">
                    <span class="font-bold text-sm">"Daily Drivers"</span>
                    <span>"JavaScript/TypeScript, PHP, HTML, CSS"</span>
                </div>
                <div class="flex-1 flex flex-col">
                    <span class="font-bold text-sm">"Experiments"</span>
                    <span>"Rust, Golang, Dart, Python"</span>
                </div>
            </div>
        </div>
        <div class="flex gap-8 mt-4">
            <span class="w-1/4">"Frameworks:"</span>
            <div
                class="flex-1 border border-iceblue/10 border-separate border-dashed flex pt-1 px-2 mb-2 text-base"
            >
                <div class="flex-1 flex flex-col pr-2">
                    <span class="font-bold text-sm">"Daily Drivers"</span>
                    <span>"Laravel, Svelte/SvelteKit, React"</span>
                </div>
                <div class="flex-1 flex flex-col">
                    <span class="font-bold text-sm">"Experiments"</span>
                    <span>"Flutter, Vue"</span>
                </div>
            </div>
        </div>
        <div class="flex gap-8 mt-4">
            <span class="w-1/4">"Testing:"</span>
            <div
                class="flex-1 border border-iceblue/10 border-separate border-dashed flex pt-1 px-2 mb-2 text-base"
            >
                <div class="flex-1 flex flex-col pr-2">
                    <span class="font-bold text-sm">"Daily Drivers"</span>
                    <span>"Jest, PHPUnit, React Testing Library"</span>
                </div>
                <div class="flex-1 flex flex-col">
                    <span class="font-bold text-sm">"Experiments"</span>
                    <span>"Playwright"</span>
                </div>
            </div>
        </div>
        <div class="flex gap-8 mt-4">
            <span class="w-1/4">"Data Stores:"</span>
            <div
                class="flex-1 border border-iceblue/10 border-separate border-dashed flex pt-1 px-2 mb-2 text-base"
            >
                <div class="flex-1 flex flex-col pr-2">
                    <span class="font-bold text-sm">"Daily Drivers"</span>
                    <span>"AWS RDS, MySQL, Redis"</span>
                </div>
                <div class="flex-1 flex flex-col">
                    <span class="font-bold text-sm">"Experiments"</span>
                    <span>"SQLite, MongoDB, PostgreSQL"</span>
                </div>
            </div>
        </div>
        <div class="flex gap-8 mt-4">
            <span class="w-1/4">"Cloud/Deployment:"</span>
            <div
                class="flex-1 border border-iceblue/10 border-separate border-dashed flex pt-1 px-2 mb-2 text-base"
            >
                <div class="flex-1 flex flex-col pr-2">
                    <span class="font-bold text-sm">"Daily Drivers"</span>
                    <span>"AWS, DigitalOcean"</span>
                </div>
                <div class="flex-1 flex flex-col">
                    <span class="font-bold text-sm">"Experiments"</span>
                    <span>"Firebase"</span>
                </div>
            </div>
        </div>
        <div class="flex gap-8 mt-4">
            <span class="w-1/4">"AI/Data Science:"</span>
            <div
                class="flex-1 border border-iceblue/10 border-separate border-dashed flex pt-1 px-2 mb-2 text-base"
            >
                <div class="flex-1 flex flex-col pr-2">
                    <span class="font-bold text-sm">"Daily Drivers"</span>
                    <span>"-"</span>
                </div>
                <div class="flex-1 flex flex-col">
                    <span class="font-bold text-sm">"Experiments"</span>
                    <span>"Tensorflow"</span>
                </div>
            </div>
        </div>
        </div>
           );

        return Ok(body.to_string());
    }
}
