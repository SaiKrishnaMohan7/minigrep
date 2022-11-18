use std::env;
use std::fs;

fn main() {
    // it is convention to bring parent module into scope rather than the function
    let args: Vec<String> = env::args().collect();
    let query_string = &args[1];
    let filename = &args[2];

    println!("Search for String: {}", query_string);
    println!("In filename: {}", filename);

    let contents = match fs::read_to_string(filename) {
        Ok(file_contents) => file_contents,
        Err(error) => panic!("Error reading file contents {:?}", error)
    };

    // let contents = fs::read_to_string(filename).expect("Something went wrong reading file");

    println!("With text: \n {}", contents);
}
