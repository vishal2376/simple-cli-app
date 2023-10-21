use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Query : {}", query);
    println!("Filename : {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file.");

    println!("\nFile Content:\n{}", contents);
}
