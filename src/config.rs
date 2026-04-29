pub struct Config {
    pub pattern: String,
    pub filename: Option<String>,
    pub show_line_numbers: bool,
    pub case_insensitive: bool,
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Config, &'static str> {
        let mut pattern = None;
        let mut filename = None;
        let mut show_line_numbers = false;
        let mut case_insensitive = false;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-n" => show_line_numbers = true,
                "-i" => case_insensitive = true,
                arg if pattern.is_none() => pattern = Some(arg.to_string()),
                arg => filename = Some(arg.to_string()),
            }
            i += 1;
        }

        match pattern {
            Some(pattern) => Ok(Config {
                pattern,
                filename,
                show_line_numbers,
                case_insensitive,
            }),
            _ => Err("Usage: scour [-i][-n] <pattern> <file>"),
        }
    }
}
