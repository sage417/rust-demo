use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;
    let new_contents: String;
    // println!("With text:\n{contents}");

    let results = if config.insensitive == true {
        new_contents = contents.to_lowercase();
        search(&config.query.to_lowercase(), &new_contents)
    } else {
        search(&config.query, &contents)
    };

    for ele in results {
        eprintln!("{ele}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub insensitive: bool,
}

impl Config {
    // fn new(args: &[String]) -> Config {
    //     if args.len() < 3 {
    //         panic!("not enough arguments")
    //     }

    //     let query = args[1].clone();
    //     let file_path = args[2].clone();

    //     Config { query, file_path }
    // }
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }
        // let query = args[1];
        // let file_path = args[2];
        args.next();

        let query = match args.next() {
            Some(str) => str,
            None => return Err("not enough arguments"),
        };

        let file_path = match args.next() {
            Some(str) => str,
            None => return Err("not enough arguments"),
        };

        let case_sensitive_flag = env::var("CASE_SENSITIVE").ok();
        let case_sensitive = match case_sensitive_flag.as_deref() {
            None | Some("0") | Some("false") => false,
            Some(_) => true,
        };

        // let case_sensitive = !matches!(case_sensitive_flag.as_deref(), None | Some("0") | Some("false"));

        Ok(Config {
            query,
            file_path,
            insensitive: case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search2<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search2(query, contents));
    }
}
