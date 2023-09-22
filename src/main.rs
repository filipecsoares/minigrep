use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parce_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Something went wrong reading the file");
    println!("Here is the contents: \n\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parce_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path }
}
