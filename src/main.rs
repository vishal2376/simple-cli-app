use simple_cli_app::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem Parsing Arguments : {}", err);
        process::exit(1)
    });

    println!("Query : {}", config.query);
    println!("Filename : {}", config.filename);

    if let Err(e) = simple_cli_app::run(config) {
        eprintln!("Application Error : {}", e);
        process::exit(1);
    }
}
