use std::env;
use std::fs;
use std::process;

pub fn search<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

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

#[cfg(test)]
mod tests {
    use crate::search;
    
    #[test]
    fn test_search_find_matching_lines() {
        let pattern = "fn";
        let content = "\
            fn main() {
                println!(\"Hello\");
            }
            let x = 1;
        ";
        
        assert_eq!(search(pattern, content), vec!["fn main() {"]);
    }
 
    #[test]
    fn test_search_no_match() {
        let pattern = "zzz";
        let content = "hello\nworld";
        assert_eq!(search(pattern, content), Vec::<&str>::new());
    }
}
