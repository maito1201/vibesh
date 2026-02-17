mod config;
mod executor;
mod filter;

use config::Config;
use executor::ShellExecutor;
use filter::CommandFilter;
use std::io::{self, Write};

fn main() {
    let config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    };

    let executor = ShellExecutor::new(config.shell.clone());
    let filter = CommandFilter::new(&config);

    println!("vibesh shell (backend: {})", config.shell);
    println!("Type 'exit' to quit");

    loop {
        print!("> ");
        if let Err(e) = io::stdout().flush() {
            eprintln!("Failed to flush stdout: {}", e);
            continue;
        }

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "exit" {
            break;
        }

        if !filter.is_allowed(input) {
            println!("this command is not allowed");
            continue;
        }

        if let Err(e) = executor.execute(input) {
            eprintln!("Error: {}", e);
        }
    }
}
