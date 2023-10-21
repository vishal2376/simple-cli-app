use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem Parsing Arguments : {}", err);
        process::exit(1)
    });

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
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not Enought Arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
