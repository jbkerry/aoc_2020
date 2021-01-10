use itertools::Itertools;

struct PasswordAuth {
    bounds: Vec<usize>,
    letter: String,
    password: String,
}

impl PasswordAuth {
    fn new(auth_data: &str) -> PasswordAuth {
        let parts: Vec<&str> = auth_data.split(" ").collect();
        let bounds: Vec<usize> = parts[0]
            .split("-")
            .map(|v| v.parse().unwrap())
            .collect();
        let letter = parts[1].replace(":", "");
        let password = parts[2].to_string();

        PasswordAuth {
            bounds,
            letter,
            password
        }
    }
}

pub fn run(lines: Vec<String>, part: usize) -> Result<(), &'static str> {
    let mut valid= 0;
    for passwd_data in lines.iter() {
        let password_auth = PasswordAuth::new(passwd_data);

        let is_valid = if part == 1 {
            is_valid_part1(password_auth)
        } else {
            is_valid_part2(password_auth)
        };

        if is_valid {
            valid += 1;
        }
    }

    println!("number of valid passwords = {}", valid);

    Ok(())
}

fn is_valid_part1(password_auth: PasswordAuth) -> bool {
    let letter_occur = password_auth.password.matches(&password_auth.letter).count();
    letter_occur >= password_auth.bounds[0] && letter_occur <= password_auth.bounds[1]
}

fn is_valid_part2(password_auth: PasswordAuth) -> bool {
    let letters: Vec<_> = password_auth.password.chars().collect();
    let lower_matches = letters[password_auth.bounds[0] - 1]
        .to_string() == password_auth.letter;
    let upper_matches = letters[password_auth.bounds[1] - 1]
        .to_string() == password_auth.letter;
    let vec = [lower_matches, upper_matches];

    !vec.iter().all_equal()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_password_details() {
        let password_auth = PasswordAuth::new("1-5 c: gatcdf");
        assert_eq!(vec![1, 5], password_auth.bounds);
        assert_eq!("c".to_string(), password_auth.letter);
        assert_eq!("gatcdf".to_string(), password_auth.password);
    }

    #[test]
    fn detects_valid_password_part1() {
        let password_auth = PasswordAuth::new("2-5 a: bgahjaba");
        assert!(is_valid_part1(password_auth));
    }

    #[test]
    fn detects_invalid_password_part1_not_enough() {
        let password_auth = PasswordAuth::new("3-5 g: bcgagth");
        assert!(!is_valid_part1(password_auth));
    }

    #[test]
    fn detects_invalid_password_part1_too_many() {
        let password_auth = PasswordAuth::new("1-3 c: aghccsdccd");
        assert!(!is_valid_part1(password_auth));
    }

    #[test]
    fn detects_valid_password_part2() {
        let password_auth = PasswordAuth::new("2-5 a: badaff");
        assert!(is_valid_part2(password_auth));
    }

    #[test]
    fn detects_invalid_password_part2_none_match() {
        let password_auth = PasswordAuth::new("3-6 t: ttatsdf");
        assert!(!is_valid_part2(password_auth));
    }

    #[test]
    fn detects_invalid_password_part2_both_match() {
        let password_auth = PasswordAuth::new("2-4 x: gxhxasd");
        assert!(!is_valid_part2(password_auth));
    }
}