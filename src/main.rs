mod config;
mod search;

use crate::config::Config;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::from_args(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let content = fs::read_to_string(&config.filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", config.filename, err);
        process::exit(1);
    });

    if config.show_line_numbers {
        for (n, line) in search::search_with_line_numbers(&config.pattern, &content) {
            println!("{}: {}", n, line);
        }
    } else if config.case_insensitive {
        for line in search::search_case_insensitive(&config.pattern, &content) {
            println!("{}", line);
        }
    } else {
        for line in search::search(&config.pattern, &content) {
            println!("{}", line);
        }
    }
}
