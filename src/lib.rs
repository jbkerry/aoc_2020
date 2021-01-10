use std::error::Error;
use std::fs;

mod day1;
mod day2;
mod day3;


pub fn get_lines_from_file(filepath: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(filepath)?;
    let lines = contents
        .lines()
        .map(|l| l.to_string())
        .collect();

    Ok(lines)
}

pub fn run(lines:Vec<String>, day: i8, part: usize) -> Result<(), &'static str> {
    let func = match day {
        1 => day1::run,
        2 => day2::run,
        3 => day3::run,
        _ => return Err("No function defined for this day"),
    };
    func(lines, part)?;
    Ok(())
}

