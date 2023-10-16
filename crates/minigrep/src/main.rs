use std::{env, process};
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(2);
    }
}
