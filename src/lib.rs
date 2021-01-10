use std::env;
use std::error::Error;
use std::fs;

mod day1;
mod day2;
mod day3;

pub struct RunConfig {
    pub day: usize,
    pub part: usize,
    pub filepath: String,
}

impl RunConfig {
    pub fn new(mut args: env::Args) -> Result<RunConfig, &'static str> {
        args.next();
        let day: usize = match args.next() {
            Some(arg) => arg.parse().unwrap(),
            None => return Err("Didn't get a day number"),
        };

        let part: usize = match args.next() {
            Some(arg) => arg.parse().unwrap(),
            None => return Err("Didn't get a part number"),
        };

        let filepath = format!("./day{}.txt", day);

        Ok(RunConfig {
            day,
            part,
            filepath,
        })
    }
}


pub fn get_lines_from_file(filepath: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(filepath)?;
    let lines = contents
        .lines()
        .map(|l| l.to_string())
        .collect();

    Ok(lines)
}

pub fn run(lines:Vec<String>, day: usize, part: usize) -> Result<(), &'static str> {
    let func = match day {
        1 => day1::run,
        2 => day2::run,
        3 => day3::run,
        _ => return Err("No function defined for this day"),
    };
    func(lines, part)?;
    Ok(())
}

