use std::env;
use std::process;

use Silicon;
use Silicon::Config;
use std::path::PathBuf;

mod engine;
mod parse;
mod lex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        engine::run(PathBuf::from(&args[1]))
    }
    else {
        println!("Please specify the path to the main script!");
        process::exit(1);
    }

    println!("Program execution finished successfully.");
}
