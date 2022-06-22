use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Missing arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config { query, filename, case_sensitive });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let output = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    for line in output {
        println!("{}", line);
    }

    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    return results;
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut output = Vec::new();

    for line in contents.lines() {
        if !line.to_lowercase().contains(&query) { continue; }
        output.push(line);
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_sensitive(query, contents)
        );
    }
}