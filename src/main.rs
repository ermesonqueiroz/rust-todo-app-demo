use std::{env, process};

mod commands;

use commands::{AddCommand, Command, ListCommand};

fn main() -> () {
    let args: Vec<String> = env::args().collect();

    let command = args.get(1).unwrap_or_else(|| {
        println!("Hello, World!");
        process::exit(0);
    });

    let exit_code = match command.as_str() {
        "add" => AddCommand::new(args).handle(),
        "list" => ListCommand::new().handle(),
        _ => {
            println!("Unknown command");
            1
        }
    };

    process::exit(exit_code);
}
