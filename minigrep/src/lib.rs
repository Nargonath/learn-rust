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

    println!("With text:\n{}", contents);

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn config_new_errors_when_less_than_three_args() {
        let args = vec![String::from("test-program")];
        let expected = "not enough arguments";

        let result = super::Config::new(&args);

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

        let result = super::Config::new(&args);

        assert!(result.is_ok(), "should be Ok()");

        let expected = super::Config {
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
}
