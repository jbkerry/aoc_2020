use std::error::Error;
use std::fs;
use std::process;

use itertools::Itertools;

pub fn run(filepath: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filepath)?;
    let lines = contents.lines().collect();
    let group_adding_to_values = find_group(lines, 2, 2020);
    let result: i32 = match group_adding_to_values {
        Some(r) => r.iter().product(),
        None => {
            println!("Not found");
            process::exit(0);
        },
    };
    println!("result = {}", result);

    Ok(())
}

fn find_group(lines: Vec<&str>, combs: usize, total: i32) -> Option<Vec<i32>> {
    let lines_as_nums = lines
        .iter()
        .map(|x| x.parse().unwrap());

    lines_as_nums
        .combinations(combs)
        .filter(|pair| pair.iter().sum::<i32>() == total)
        .next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_on_2_comb() {
        let lines = vec!["4", "5", "1", "3", "9"];
        assert_eq!(vec![5, 9], find_group(lines, 2, 14).unwrap());
    }

    #[test]
    fn sums_on_3_comb() {
        let lines = vec!["10", "20", "50", "60", "95", "150"];
        assert_eq!(vec![10, 95, 150], find_group(lines, 3, 255).unwrap());
    }
}