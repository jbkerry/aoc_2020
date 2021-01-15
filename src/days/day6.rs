use std::collections::HashSet;

pub fn run(lines: Vec<String>, part: usize) -> Result<(), &'static str> {
    let total_questions: usize = lines
        .iter()
        .map(|x| count_unique_chars(x))
        .sum();
    if part == 1 {
        println!("Total number of questions = {}", total_questions);
    }
    Ok(())
}

fn count_unique_chars(chars_str: &str) -> usize {
    let unique_chars: HashSet<char> = chars_str.replace("\n", "").chars().collect();
    unique_chars.iter().count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn counts_number_of_unique_chars() {
        let questions_answered = "abcx\nabcy\nabcz\n";
        assert_eq!(6, count_unique_chars(&questions_answered));
    }
}