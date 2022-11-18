use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("Search for String: {}", config.query_string);
    println!("In filename: {}", config.filename);

    let contents = match fs::read_to_string(config.filename) {
        Ok(file_contents) => file_contents,
        Err(error) => panic!("Error reading file contents {:?}", error)
    };

    println!("With text: \n {}", contents);
}

struct Config {
    query_string: String,
    filename: String,
}
fn parse_config(args: &[String]) -> Config {
    let query_string = args[1].clone();
    let filename = args[2].clone();

    Config {
        query_string,
        filename,
    }
}