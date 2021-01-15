use std::collections::{HashSet, HashMap};

use itertools::Itertools;

pub fn run(lines: Vec<String>, part: usize) -> Result<(), &'static str> {
    let count_criteria: fn(&String) -> usize = if part == 1 {
        |x| count_unique_chars(x)
    } else {
        |x| num_chars_in_all_lines(x)
    };

    let total_questions: usize = lines
        .iter()
        .map(count_criteria)
        .sum();

    println!("Total number of questions = {}", total_questions);

    Ok(())
}

fn count_unique_chars(chars_str: &str) -> usize {
    let unique_chars: HashSet<&str> = chars_str.matches(char::is_alphabetic).collect();
    unique_chars.iter().count()
}

fn num_chars_in_all_lines(chars_str: &str) -> usize {
    let num_of_people = chars_str.lines().count();
    let mut question_counts = count_char_occurrences(chars_str);
    question_counts.retain(|_, v| v == &num_of_people);
    question_counts.len()
}

fn count_char_occurrences(chars_str: &str) -> HashMap<&str, usize> {
    let questions: Vec<&str> = chars_str.matches(char::is_alphabetic).collect();
    let question_counts: HashMap<&str, usize> = questions.into_iter().counts();
    question_counts
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn counts_number_of_unique_chars() {
        let questions_answered = "abcx\nabcy\nabcz\n";
        assert_eq!(6, count_unique_chars(&questions_answered));
    }

    #[test]
    fn counts_occurences() {
        let questions_answered = "abcx\nabcy\nabcz\n";
        let mut expected_result: HashMap<&str, usize> = HashMap::new();
        expected_result.insert("a", 3);
        expected_result.insert("b", 3);
        expected_result.insert("c", 3);
        expected_result.insert("x", 1);
        expected_result.insert("y", 1);
        expected_result.insert("z", 1);
        assert_eq!(expected_result, count_char_occurrences(questions_answered));
    }

    #[test]
    fn counts_chars_on_all_lines() {
        let questions_answered = "abcx\nabcy\nabcz\n";
        assert_eq!(3, num_chars_in_all_lines(questions_answered));
    }
}