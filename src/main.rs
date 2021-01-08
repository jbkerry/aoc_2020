use std::process;

fn main() {
    if let Err(e) = aoc_2020::run("./day1.txt") {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
