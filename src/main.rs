mod config;
mod search;

use crate::config::Config;
use std::fs;
use std::io::Read;
use std::process;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::from_args(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let content = match &config.filename {
        Some(filename) => fs::read_to_string(filename).unwrap_or_else(|err| {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }),
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).unwrap_or_else(|err| {
                eprintln!("Error reading stdin: {}", err);
                process::exit(1);
            });
            buf
        }
    };

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
