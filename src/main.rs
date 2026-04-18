use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: scour <pattern> <file>");
        process::exit(1);
    }
    
    let pattern = &args[1];
    let filename = &args[2];
    
    let content = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        process::exit(1);
    });
    
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}
