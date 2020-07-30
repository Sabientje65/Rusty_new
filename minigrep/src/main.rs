mod constants;

use std::env;
use std::process;

use minigrep::Config;
use constants::error_codes;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!(" Problem parsing arguments: {}", err);
        process::exit(error_codes::INSUFFICIENT_ARGS);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file);

    if let Err(e) = minigrep::run(&config) {
        println!("Application error: {}", e);
        process::exit(error_codes::APP_ERROR);
    }
}

