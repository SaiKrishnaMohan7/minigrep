use std::env;

fn main() {
    // it is convention to bring parent module into scope rather than the function
    let args: Vec<String> = env::args().collect();
    let query_string = &args[1];
    let filename = &args[2];

    println!("Search for String: {}", query_string);
    println!("In filename: {}", filename);
}
