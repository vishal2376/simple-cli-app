use simple_cli_app::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem Parsing Arguments : {}", err);
        process::exit(1)
    });

    println!("Query : {}", config.query);
    println!("Filename : {}", config.filename);

    if let Err(e) = simple_cli_app::run(config) {
        println!("Application Error : {}", e);
        process::exit(1);
    }
}
