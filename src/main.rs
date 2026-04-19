use std::env;
use std::fs;
use std::process;

pub fn search<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

pub fn search_with_line_numbers<'a>(pattern: &str, content: &'a str) -> Vec<(usize, &'a str)> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(pattern))
        .map(|(i, line)| (i + 1, line))
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

    for line in search(pattern, &content) {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use crate::{search, search_with_line_numbers};

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

    #[test]
    fn test_search_with_line_numbers() {
        let pattern = "fn";
        let content = r#"let x = 1;
fn main() {
    println!("hello");
}"#;
        assert_eq!(search_with_line_numbers(pattern, content), vec![(2, "fn main() {")]);
    }

}
