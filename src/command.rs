pub trait TerminalCommand {
    fn run(&self) -> Result<String, String>;
}

pub struct AddCommand {
    pub args: Vec<String>,
}

impl TerminalCommand for AddCommand {
    fn run(&self) -> Result<String, String> {
        if self.args.len() != 2 {
            return Err(format!("add_command: Invalid number of arguments"));
        }

        let numbers: Vec<i32> = self
            .args
            .iter()
            .map(|arg| arg.parse::<i32>().unwrap())
            .collect();

        let result = numbers[0] + numbers[1];
        Ok(result.to_string())
    }
}
