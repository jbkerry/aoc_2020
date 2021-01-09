use std::process;

fn main() {
    let day = 2;
    let filepath = format!("./day{}.txt", day);
    let lines = aoc_2020::get_lines_from_file(&filepath)
        .unwrap_or_else(|err| {
            eprintln!("Problem reading file: {}", err);
            process::exit(1);
        });

    if let Err(e) = aoc_2020::run(lines, day, 2) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
