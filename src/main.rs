use std::process;

fn main() {
    let lines = aoc_2020::get_lines_from_file("./day2.txt")
        .unwrap_or_else(|err| {
            eprintln!("Problem reading file: {}", err);
            process::exit(1);
        });

    if let Err(e) = aoc_2020::day2(lines) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
