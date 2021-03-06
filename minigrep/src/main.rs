mod constants;

use std::env;
use std::process;

use minigrep::Config;
use constants::error_codes;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(error_codes::INSUFFICIENT_ARGS);
    });

    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application error: {}", e);
        
        process::exit(error_codes::APP_ERROR);
    }
}

