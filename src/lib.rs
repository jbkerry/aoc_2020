use std::error::Error;
use std::fs;

use itertools::Itertools;

pub fn get_lines_from_file(filepath: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(filepath)?;
    let lines = contents
        .lines()
        .map(|l| l.to_string())
        .collect();

    Ok(lines)
}

pub fn day1(lines: Vec<String>) -> Result<(), &'static str> {
    let group_adding_to_values = find_group(lines, 2, 2020);
    let result: i32 = match group_adding_to_values {
        Some(r) => r.iter().product(),
        None => return Err("Didn't get a file name"),
    };
    println!("result = {}", result);

    Ok(())
}

fn find_group(lines: Vec<String>, combs: usize, total: i32) -> Option<Vec<i32>> {
    let lines_as_nums = lines
        .iter()
        .map(|x| x.parse().unwrap());

    lines_as_nums
        .combinations(combs)
        .filter(|pair| pair.iter().sum::<i32>() == total)
        .next()
}

pub fn day2(lines: Vec<String>) -> Result<(), &'static str> {
    let mut valid= 0;
    for passwd_data in lines.iter() {
        let (bounds, letter, password) = retrieve_password_data(passwd_data);
        let is_valid = is_valid_part2(bounds, letter, password);
        if is_valid {
            valid += 1;
        }
    }

    println!("{}", valid);

    Ok(())
}

fn retrieve_password_data(data: &String) -> (Vec<i32>, String, &str) {
    let parts: Vec<&str> = data.split(" ").collect();
    let bounds: Vec<i32> = parts[0].split("-").map(|v| v.parse().unwrap()).collect();
    let letter = parts[1].replace(":", "");
    let password = parts[2];

    (bounds, letter, password)
}

fn is_valid_part1(bounds: Vec<i32>, letter: String, password: &str) -> bool {
    let letter_occur = password.matches(&letter).count() as i32;
    if letter_occur >= bounds[0] && letter_occur <= bounds[1] {
        return true
    }
    false
}

fn is_valid_part2(bounds: Vec<i32>, letter: String, password: &str) -> bool {
    let letters: Vec<_> = password.chars().collect();
    let lower_matches = letters[(bounds[0] - 1) as usize].to_string() == letter;
    let upper_matches = letters[(bounds[1] - 1) as usize].to_string() == letter;
    let vec = vec![lower_matches, upper_matches];
    if vec.iter().any(|x| *x) && !vec.iter().all(|x| *x) {
        return true
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_on_2_comb() {
        let lines = vec![
            "4".to_string(),
            "5".to_string(),
            "1".to_string(),
            "3".to_string(),
            "9".to_string()
        ];
        assert_eq!(vec![5, 9], find_group(lines, 2, 14).unwrap());
    }

    #[test]
    fn sums_on_3_comb() {
        let lines = vec![
            "10".to_string(),
            "20".to_string(),
            "50".to_string(),
            "60".to_string(),
            "95".to_string(),
            "150".to_string()
        ];
        assert_eq!(vec![10, 95, 150], find_group(lines, 3, 255).unwrap());
    }

    #[test]
    fn splits_password_details() {
        let data = "1-5 c: gatcdf".to_string();
        assert_eq!((vec![1, 5], "c".to_string(), "gatcdf"), retrieve_password_data(&data));
    }
}