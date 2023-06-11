use std::{env, process};
use n12_5n6::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = n12_5n6::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}