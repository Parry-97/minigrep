use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|_err| {
        eprintln!("Problem parsing arguments: {_err}");
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);
    // dbg!(args);
    if let Err(_e) = minigrep::run(config) {
        eprintln!("Application error: {_e}");
        process::exit(1);
    }
}
