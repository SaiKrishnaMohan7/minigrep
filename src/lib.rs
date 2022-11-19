use std::error::Error;
use std::fs;

pub struct Config {
    pub query_string: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query_string = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query_string,
            filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query_string, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query_string: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines_found = vec![];

    for line in contents.lines() {
        if line.contains(query_string) {
            lines_found.push(line);
        }
    }

    lines_found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
