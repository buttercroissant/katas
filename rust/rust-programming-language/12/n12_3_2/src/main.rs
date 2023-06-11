use std::{env, process};
use n12_3_2::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    if let Err(e) = n12_3_2::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

