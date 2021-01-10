use std::env;
use std::process;

use aoc_2020::RunConfig;

fn main() {
    let config = RunConfig::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let lines = aoc_2020::get_lines_from_file(&config.filepath)
        .unwrap_or_else(|err| {
            eprintln!("Problem reading file: {}", err);
            process::exit(1);
        });

    if let Err(e) = aoc_2020::run(lines, config.day, config.part) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
