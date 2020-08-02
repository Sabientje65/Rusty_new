use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    
    for matched_line in results {
        println!("{}", matched_line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
           matches.push(line) 
        }
    }

    matches
}

fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

impl Config {
    const POS_QUERY: usize = 1;
    const POS_FILE: usize = 2;
    
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Minimum of 2 arguments required");
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        let config = Config {
            query: args[Config::POS_QUERY].clone(),
            file: args[Config::POS_FILE].clone(),
            case_sensitive
        };
        
        Ok(config)
    }
}

pub struct Config {
    pub query: String,
    pub file: String,
    pub case_sensitive: bool
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
Pick three.
Duct tape.";

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

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}


