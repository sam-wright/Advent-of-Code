extern crate regex;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Config {
    min: i32,
    max: i32,
    val: char,
    password: String,
}

impl Config {
    pub fn validate(&self) -> bool {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.val {
                count += 1;
            }
        }

        count >= self.min && count <= self.max
    }

    pub fn validate2(&self) -> bool {
        (self.password.chars().nth((self.min - 1) as usize).unwrap() == self.val
            && self.password.chars().nth((self.max - 1) as usize).unwrap() != self.val)
            || (self.password.chars().nth((self.max - 1) as usize).unwrap() == self.val
                && self.password.chars().nth((self.min - 1) as usize).unwrap() != self.val)
    }
}

pub fn deserialize(input: &str) -> Config {
    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let caps = re.captures(input).unwrap();

    Config {
        min: caps[1].parse().unwrap(),
        max: caps[2].parse().unwrap(),
        val: caps[3].parse().unwrap(),
        password: caps[4].to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    pub fn parse_input() -> Vec<String> {
        let mut contents = String::new();
        let mut file = File::open("input.txt").unwrap();
        file.read_to_string(&mut contents).unwrap();

        let collection: Vec<String> = contents.split('\n').map(|x| x.to_string()).collect();
        collection
    }
    #[test]
    fn test_deserialize() {
        let input = "1-3 a: abcde";
        assert_eq!(
            deserialize(input),
            Config {
                min: 1,
                max: 3,
                val: 'a',
                password: "abcde".to_string(),
            }
        );
    }

    #[test]
    fn part_1_example() {
        let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

        for item in input {
            if deserialize(item).validate() {
                println!("pass");
            } else {
                println!("fail");
            }
        }
    }

    #[test]
    fn part_1_solution() {
        let input = parse_input();

        let pass = input
            .iter()
            .fold(0, |x, y| if deserialize(y).validate() { x + 1 } else { x });

        println!("Pass = {}", pass);
        assert_eq!(640, pass);
    }

    #[test]
    fn part_2_example() {
        let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

        for item in input {
            if deserialize(item).validate2() {
                println!("pass");
            } else {
                println!("fail");
            }
        }
    }

    #[test]
    fn part_2_solution() {
        let input = parse_input();

        let pass = input
            .iter()
            .fold(0, |x, y| if deserialize(y).validate2() { x + 1 } else { x });

        println!("Pass = {}", pass);
        assert_eq!(472, pass);
    }
}
