use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &content) {
        println!("\n{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not Enought Arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "feat";
        let content = "\
Rust is a powerful language.
It is most loved language.
It has most uniqe features.";
        assert_eq!(vec!["It has most uniqe features."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "lAnG";
        let content = "\
Rust is a powerful language.
It is most loved language.
It has most uniqe features.";
        assert_eq!(
            vec!["Rust is a powerful language.", "It is most loved language."],
            search_insensitive(query, content)
        );
    }
}
