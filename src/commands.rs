use std::fs;
use std::io::Write;

pub trait Command {
    fn handle(&self) -> i32;
}

pub struct AddCommand {
    args: Vec<String>,
}

impl AddCommand {
    pub fn new(args: Vec<String>) -> Self {
        AddCommand { args }
    }
}

impl Command for AddCommand {
    fn handle(&self) -> i32 {
        if let Some(description) = &self.args.get(2) {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("Storage.txt")
                .expect("File not found");

            writeln!(file, "{description}").expect("File not writable.");

            println!("Todo added");

            0
        } else {
            println!("Description is required");
            1
        }
    }
}

pub struct ListCommand {}

impl ListCommand {
    pub fn new() -> Self {
        ListCommand {}
    }
}

impl Command for ListCommand {
    fn handle(&self) -> i32 {
        let contents = fs::read_to_string("Storage.txt").expect("File not found.");

        println!("{contents}");
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_command() {
        let args: Vec<String> = vec![
            "todo".to_string(),
            "add".to_string(),
            "My todo 4".to_string(),
        ];

        let exit_code = AddCommand::new(args).handle();

        assert_eq!(0, exit_code);
    }

    #[test]
    fn list_command() {
        let exit_code = ListCommand::new().handle();

        assert_eq!(0, exit_code);
    }
}
