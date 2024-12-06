use aoc::cli;
use std::{env, process};

fn main() {
    if let Err(e) = cli::run(env::args()) {
        eprintln!("{:?}", e);
        process::exit(1);
    }
}
