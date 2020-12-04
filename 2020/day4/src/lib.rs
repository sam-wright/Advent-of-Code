use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

extern crate regex;
use regex::Regex;

// pub struct Passport {}
type Passport = HashMap<String, String>;

pub fn read_input(filename: &str) -> Vec<String> {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let collection: Vec<String> = contents.split("\n\n").map(|x| x.to_string()).collect();
    collection
}

pub fn parse_input(input: Vec<String>) -> Vec<Passport> {
    let re = Regex::new(r"(\w+):(\S+)").unwrap();

    let mut output = Vec::new();

    for line in &input {
        let mut passport = HashMap::new();
        for cap in re.captures_iter(&line) {
            passport.insert(cap[1].to_string(), cap[2].to_string());
        }
        output.push(passport);
    }

    output
}

pub fn validate(passport: &Passport) -> bool {
    passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid")
}

fn validate_byr(input: &str) -> bool {
    let number: i32 = match input.parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    number >= 1920 && number <= 2002 && input.len() == 4
}

fn validate_iyr(input: &str) -> bool {
    let number: i32 = match input.parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    number >= 2010 && number <= 2020 && input.len() == 4
}

fn validate_eyr(input: &str) -> bool {
    let number: i32 = match input.parse() {
        Ok(v) => v,
        Err(_) => return false,
    };
    number >= 2020 && number <= 2030 && input.len() == 4
}

fn validate_hgt(input: &str) -> bool {
    let cm_re = Regex::new(r"(\d+)cm").unwrap();
    let in_re = Regex::new(r"(\d+)in").unwrap();

    if input.contains("cm") {
        let caps = cm_re.captures(input).unwrap();
        let number: i32 = match caps[1].parse() {
            Ok(v) => v,
            Err(_) => return false,
        };
        number >= 150 && number <= 193
    } else if input.contains("in") {
        let caps = in_re.captures(input).unwrap();
        let number: i32 = match caps[1].parse() {
            Ok(v) => v,
            Err(_) => return false,
        };
        number >= 59 && number <= 76
    } else {
        false
    }
}

fn validate_hcl(input: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    re.is_match(input)
}

fn validate_ecl(input: &str) -> bool {
    match input {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

fn validate_pid(input: &str) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(input)
}

fn extra_validate(passport: &Passport) -> bool {
    for (key, value) in passport {
        if match key.as_str() {
            "byr" => validate_byr(&value),
            "iyr" => validate_iyr(&value),
            "eyr" => validate_eyr(&value),
            "hgt" => validate_hgt(&value),
            "hcl" => validate_hcl(&value),
            "ecl" => validate_ecl(&value),
            "pid" => validate_pid(&value),
            "cid" => true,
            _ => false,
        } {
            continue;
        } else {
            return false;
        }
    }
    true
}

pub fn count_valid(passports: Vec<Passport>) -> i32 {
    let mut valid = 0;

    for passport in passports {
        if validate(&passport) {
            valid += 1;
        }
    }

    println!("Final valid count: {}", valid);
    valid
}

pub fn count_extra_valid(passports: Vec<Passport>) -> i32 {
    let mut valid = 0;

    for passport in passports {
        if validate(&passport) && extra_validate(&passport) {
            valid += 1;
        }
    }

    println!("Final valid count: {}", valid);
    valid
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_1_example() {
        let input = read_input("example1.txt");
        let parsed_data = parse_input(input);
        assert_eq!(2, count_valid(parsed_data));
    }

    #[test]
    fn part_1_solution() {
        let input = read_input("input.txt");
        let parsed_data = parse_input(input);
        assert_eq!(213, count_valid(parsed_data));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(true, validate_byr("2002"));
        assert_eq!(false, validate_byr("2003"));
        assert_eq!(false, validate_byr("b"));

        assert_eq!(true, validate_iyr("2012"));
        assert_eq!(false, validate_iyr("2a012"));

        assert_eq!(true, validate_hgt("60in"));
        assert_eq!(true, validate_hgt("190cm"));
        assert_eq!(false, validate_hgt("190in"));
        assert_eq!(false, validate_hgt("190"));

        assert_eq!(true, validate_hcl("#123abc"));
        assert_eq!(false, validate_hcl("#123abz"));
        assert_eq!(false, validate_hcl("123abc"));

        assert_eq!(true, validate_ecl("brn"));
        assert_eq!(false, validate_ecl("wat"));

        assert_eq!(true, validate_pid("000000001"));
        assert_eq!(false, validate_pid("0123456789"));
    }

    #[test]
    fn part_2_valid_examples() {
        let input = read_input("example2_valid.txt");

        let parsed_data = parse_input(input);
        assert_eq!(4, count_extra_valid(parsed_data))
    }

    #[test]
    fn part_2_invalid_examples() {
        let input = read_input("example2_invalid.txt");

        let parsed_data = parse_input(input);
        assert_eq!(0, count_extra_valid(parsed_data))
    }

    #[test]
    fn part_2_solution() {
        let input = read_input("input.txt");
        let parsed_data = parse_input(input);
        assert_eq!(147, count_extra_valid(parsed_data));
    }
}
