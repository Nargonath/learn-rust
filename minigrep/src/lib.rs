use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(value) => value,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(value) => value,
            None => return Err("Didn't get the filename"),
        };

        let case_insensitive_flag = args.next();
        let is_case_insensitive_flag = case_insensitive_flag.is_some()
            && case_insensitive_flag.unwrap() == "--case-insensitive";

        let case_sensitive = !is_case_insensitive_flag && env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for result in results {
        println!("{}", result);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn config_new_success() {
    //     let query = String::from("query");
    //     let filename = String::from("poem.txt");
    //     let args = vec![
    //         String::from("test-program"),
    //         query.clone(),
    //         filename.clone(),
    //     ];

    //     let result = Config::new(&args);

    //     assert!(result.is_ok(), "should be Ok()");

    //     let expected = Config {
    //         query: query.clone(),
    //         filename: filename.clone(),
    //         case_sensitive: true,
    //     };
    //     let actual = result.unwrap();

    //     assert_eq!(
    //         expected, actual,
    //         "should return Config with query = {} and filename = {}",
    //         query, filename
    //     );
    // }

    // #[test]
    // fn config_case_insensitive_flag() {
    //     let query = String::from("To");
    //     let filename = String::from("test.txt");
    //     let args = vec![
    //         String::from("test-program"),
    //         query.clone(),
    //         filename.clone(),
    //         String::from("--case-insensitive"),
    //     ];

    //     let result = Config::new(&args);

    //     assert!(result.is_ok(), "should be Ok()");

    //     let expected = Config {
    //         query,
    //         filename,
    //         case_sensitive: false,
    //     };

    //     let actual = result.unwrap();

    //     assert_eq!(
    //         expected, actual,
    //         "Should return Config with case_sensitive = false"
    //     );
    // }

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
