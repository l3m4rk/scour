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

pub fn search_case_insensitive<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&pattern))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::search::*;

    #[test]
    fn test_main_with_n_flag() {
        let pattern = "fn";
        let content = r#"let x = 1;
fn main() {
    println!("hello");
}"#;

        let results = search_with_line_numbers(pattern, content);

        assert_eq!(results, vec![(2, "fn main() {")]);
    }

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

    #[test]
    fn test_search_case_insensitive() {
        let pattern = "FN";
        let content = r#"let x = 1;
fn main() {
    println!("hello");
}"#;
        assert_eq!(
            search_case_insensitive(pattern, content),
            vec!["fn main() {"]
        );
    }
}
