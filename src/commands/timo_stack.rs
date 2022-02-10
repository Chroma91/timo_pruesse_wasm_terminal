use crate::commands::TerminalCommand;

pub const TIMO_STACK_COMMAND_FLAG: &str = "--stack";

pub struct TimoStackCommand {}

impl TerminalCommand for TimoStackCommand {
    fn run(&self) -> Result<String, String> {
        return Ok("

        Languages:
        | Daily Drivers | Experiments  |
        |---------------|--------------|
        | JavaScript    | Rust         |
        | TypeScript    | Golang       |
        | PHP           | Dart         |
        |               | Python       |

        Frameworks:
        | Daily Drivers    | Experiments |
        |------------------|-------------|
        | Laravel/Symfony  | Flutter     |
        | Svelte/SvelteKit | Vue         |
        | React            |             |

        Testing:
        | Daily Drivers | Experiments |
        |---------------|-------------|
        | Jest          | Playwright  |
        | PHPUnit       |             |
        | React Testing Library       |

        Databases/Data Stores:
        | Daily Drivers | Experiments |
        |---------------|-------------|
        | AWS RDS       | SQLite      |
        | MySQL         | MongoDB     |
        | Redis         | PostgreSQL  |

        Cloud/Deployment:
        | Daily Drivers | Experiments |
        |---------------|-------------|
        | AWS           | Firebase    |
        | DigitalOcean  |             |

        AI/Data Science:
        | Daily Drivers | Experiments |
        |---------------|-------------|
        |               | Tensorflow  |

        "
        .to_string());
    }
}
