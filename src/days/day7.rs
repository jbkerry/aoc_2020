use regex::Regex;

pub fn run(lines: Vec<String>, part: usize) -> Result<(), &'static str> {
    if part == 1 {
        for line in lines {
            let parent_bag = get_parent_bag(&line)?;
            let child_bags = get_child_bags(&line);
            println!("{}, {:?}", parent_bag, child_bags);
        }
    }

    Ok(())
}

fn get_parent_bag(line: &str) -> Result<String, &'static str> {
    let re = Regex::new(r"^(?P<bag_colour>[\sa-z]+) bags contain").unwrap();
    match re.captures(&line) {
        Some(res) => Ok(res["bag_colour"].to_string()),
        None => Err("Could not determine parent bag colour"),
    }
}

fn get_child_bags(line: &str) -> Vec<String> {
    let re = Regex::new(r"(?P<child_num>\d+) (?P<bag_name>[\sa-z]+) bags?")
        .unwrap();

    re.captures_iter(&line)
        .map(|m| m["bag_name"].to_string())
        .collect()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detects_parent_bag() {
        let line = "dark purple bags contain 5 striped tan bags, 5 bright cyan bags, \
            3 dark indigo bags.";
        assert_eq!("dark purple", get_parent_bag(&line).unwrap().as_str());
    }

    #[test]
    fn raises_error_if_no_match() {
        let line = "234823 bags contain a colour";
        assert!(get_parent_bag(&line).is_err());
    }

    #[test]
    fn detects_child_bags() {
        let line = "dark purple bags contain 5 striped tan bags, 5 bright cyan bags, \
            3 dark indigo bags.";
        let expected_children = vec![
            "striped tan".to_string(),
            "bright cyan".to_string(),
            "dark indigo".to_string()
        ];
        assert_eq!(expected_children, get_child_bags(&line));
    }

    #[test]
    fn returns_empty_vector_when_no_child_bags() {
        let line = "shiny silver bags contain no other bags.";
        let expected_children: Vec<String> = vec![];
        assert_eq!(expected_children, get_child_bags(&line));
    }
}