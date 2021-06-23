use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for result in search(&config.query, &contents) {
        println!("{}", result);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut needles = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            needles.push(line);
        }
    }

    needles
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn config_new_errors_when_less_than_three_args() {
        let args = vec![String::from("test-program")];
        let expected = "not enough arguments";

        let result = Config::new(&args);

        assert!(result.is_err(), "should return an error");

        if let Err(actual) = result {
            assert_eq!(expected, actual, "does not have proper error label");
        };
    }

    #[test]
    fn config_new_success() {
        let query = String::from("query");
        let filename = String::from("poem.txt");
        let args = vec![
            String::from("test-program"),
            query.clone(),
            filename.clone(),
        ];

        let result = Config::new(&args);

        assert!(result.is_ok(), "should be Ok()");

        let expected = Config {
            query: query.clone(),
            filename: filename.clone(),
        };
        let actual = result.unwrap();

        assert_eq!(
            expected, actual,
            "should return Config with query = {} and filename = {}",
            query, filename
        );
    }

    #[test]
    fn search_sensitive_success() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_insensitive_success() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}