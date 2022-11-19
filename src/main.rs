use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

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
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
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