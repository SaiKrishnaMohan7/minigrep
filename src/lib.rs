use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query_string: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // We could have used .skip(1) on line 14 to skip the first entry

        let query_string = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query_string,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search_case_sensitive(&config.query_string, &contents)
    } else {
        search_case_insensitive(&config.query_string, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query_string: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query_string)).collect()

}

pub fn search_case_insensitive<'a>(query_string: &str, contents: &'a str) -> Vec<&'a str>{
    contents.lines().filter(|line| line.to_lowercase().contains(query_string)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "FaST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three";

        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
    }
}
