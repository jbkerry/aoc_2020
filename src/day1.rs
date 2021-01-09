use itertools::Itertools;

pub fn run(lines: Vec<String>, part: usize) -> Result<(), &'static str> {
    let group_adding_to_values = find_group(lines, part + 1, 2020);
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
        .map(|x| x.parse().expect(&format!("{} could not be parsed to int", x)));

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
}