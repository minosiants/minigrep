use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(v) => v,
            None => return Err("Didnt get the query string"),
        };

        let file_path = match args.next() {
            Some(v) => v,
            None => return Err("Didnt get file path string"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case {
        search_case_insensetive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    for line in result {
        println!("{line}")
    }
    //println!("With text\n{content}");
    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|v| v.contains(query)).collect()
}

pub fn search_case_insensetive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let q = query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&q) {
            result.push(line)
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content))
    }

    #[test]
    fn search_insensitive() {
        let query = "dUcT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensetive(query, content)
        )
    }
}
