use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    println!("search query: {}", config.query);
    println!("file path: {}", config.file_path);
    let content = fs::read_to_string(config.file_path).expect("Something went wrong reading the file");
    println!("content: {}", content);
}

fn parse_config(args: &Vec<String>) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config {
        query,
        file_path,
    }
}