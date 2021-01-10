use passport::Passport;

pub fn run(lines: Vec<String>, part: usize) -> Result<(), &'static str> {
    let valid_criteria: fn(&Passport) -> bool = if part == 1 {
        |x| x.has_required_fields()
    } else {
        |x| x.has_required_fields() && x.all_fields_are_valid()
    };

    let valid_passports = get_num_valid_passports(lines, valid_criteria);

    println!("Number of valid passports = {}", valid_passports);
    Ok(())
}

fn get_num_valid_passports(lines: Vec<String>, passport_is_valid: fn(&Passport) -> bool) -> usize {
    lines
        .iter()
        .map(|x| Passport::new(x))
        .filter(passport_is_valid)
        .count()
}

pub mod passport {
    use regex::Regex;
    use std::collections::{HashMap, HashSet};

    pub struct Passport {
        data: HashMap<String, Box<dyn FieldType>>
    }

    impl Passport {
        pub fn new(data_as_string: &String) -> Passport {
            let mut data = HashMap::new();
            let parts: Vec<Vec<&str>> = data_as_string
                .split_whitespace()
                .map(|x| x.split(":").collect())
                .collect();

            for items in parts {
                let value: Box<dyn FieldType> = match items[0] {
                    "byr" => Box::new(IntRange::new(items[1], 1920, 2002)),
                    "iyr" => Box::new(IntRange::new(items[1], 2010, 2020)),
                    "eyr" => Box::new(IntRange::new(items[1], 2020, 2030)),
                    "hgt" => Box::new(UnitRange { value: items[1].to_string() }),
                    "hcl" => Box::new( Hex { value: items[1].to_string() }),
                    "ecl" => Box::new(OfSet { value: items[1].to_string() }),
                    "pid" => Box::new(DigitSize { value: items[1].to_string(), length: 9 }),
                    _ => Box::new(Generic { })
                };
                data.insert(items[0].to_string(), value);
            }

            Passport {
                data
            }
        }

        pub fn has_required_fields(&self) -> bool {
            let required_fields: HashSet<String> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .map(|x| x.to_string())
                .collect();
            let keys_set: HashSet<String> = self.data.keys().cloned().collect();

            required_fields.is_subset(&keys_set)
        }

        pub fn all_fields_are_valid(&self) -> bool {
            self.data.values().into_iter().all(|x| x.is_valid())
        }

    }

    trait FieldType {
        fn is_valid(&self) -> bool {
            true
        }
    }

    struct IntRange {
        value: u32,
        lower_bound: u32,
        upper_bound: u32,
    }

    impl IntRange {
        fn new(value: &str, lower_bound: u32, upper_bound: u32) -> IntRange {
            let value = value.parse().unwrap();
            IntRange {
                value,
                lower_bound,
                upper_bound,
            }
        }
    }

    impl FieldType for IntRange {
        fn is_valid(&self) -> bool {
            self.lower_bound <= self.value && self.value <= self.upper_bound
        }
    }

    struct UnitRange {
        value: String,
    }

    impl FieldType for UnitRange {
        fn is_valid(&self) -> bool {
            let re = Regex::new(r"^(?P<amount>\d{2,3})(?P<unit>(cm|in))$").unwrap();
            let pat_match = re.captures(&self.value);
            if pat_match.is_none() {
                return false
            }
            let caps = pat_match.unwrap();
            let int_range = if &caps["unit"] == "cm" {
                IntRange::new(&caps["amount"], 150, 193)
            } else {
                IntRange::new(&caps["amount"], 59, 76)
            };
            int_range.is_valid()
        }
    }

    struct OfSet {
        value: String,
    }

    const VALID_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    impl FieldType for OfSet {
        fn is_valid(&self) -> bool {
            VALID_EYE_COLORS.contains(&&self.value.as_str())
        }
    }

    struct Hex {
        value: String
    }

    impl FieldType for Hex {
        fn is_valid(&self) -> bool {
            let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            re.is_match(&self.value)
        }
    }

    struct DigitSize {
        value: String,
        length: usize,
    }

    impl FieldType for DigitSize {
        fn is_valid(&self) -> bool {
            self.value.len() == self.length && self.value.parse::<u32>().is_ok()
        }
    }

    struct Generic { }

    impl FieldType for Generic {}

}