use std::error::Error;
use std::fs;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file)?;

    println!("With text:\n{}", content);

    Ok(())
}

impl Config {
    const POS_QUERY: usize = 1;
    const POS_FILE: usize = 2;
    
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Minimum of 2 arguments required");
        }

        let config = Config {
            query: args[Config::POS_QUERY].clone(),
            file: args[Config::POS_FILE].clone()
        };
        
        Ok(config)
    }
}

pub struct Config {
    pub query: String,
    pub file: String
}

