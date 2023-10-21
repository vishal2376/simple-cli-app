use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Query : {}", config.query);
    println!("Filename : {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Failed to read file.");

    println!("\nFile Content:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
