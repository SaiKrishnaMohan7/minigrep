use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args_iter_mut = env::args();
    let config = Config::new(args_iter_mut).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {}", err);

        process::exit(1);
    };
}
