use std::collections::HashSet;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(filename: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let segments = line.replace("| ", "");
        let segments = segments.split(' ');

        let words: Vec<&str> = segments.collect();

        let signal_patterns: Vec<String> = words.iter().take(10).map(|x| x.to_string()).collect();
        let digits: Vec<String> = words
            .iter()
            .rev()
            .take(4)
            .rev()
            .map(|x| x.to_string())
            .collect();

        lines.push((signal_patterns, digits));
    });

    lines
}

pub fn maybe1478(input: &str) -> bool {
    is_1(input) || is_4(input) || is_7(input) || is_8(input)
}

pub fn is_0(input: &str, six_code: &str, nine_code: &str) -> bool {
    let zero_code: HashSet<char> = input.chars().collect();

    zero_code.len() == 6
        && zero_code.intersection(&six_code.chars().collect()).count() == 5
        && zero_code.intersection(&nine_code.chars().collect()).count() == 5
}

pub fn is_1(input: &str) -> bool {
    let one_code: HashSet<char> = input.chars().collect();

    one_code.len() == 2
}

pub fn is_3(input: &str, one_code: &str, eight_code: &str) -> bool {
    let three_code: HashSet<char> = input.chars().collect();

    three_code.len() == 5
        && three_code.intersection(&one_code.chars().collect()).count() == 2
        && three_code
            .intersection(&eight_code.chars().collect())
            .count()
            == 5
}

pub fn is_4(input: &str) -> bool {
    let four_code: HashSet<char> = input.chars().collect();

    four_code.len() == 4
}

pub fn is_5(input: &str, six_code: &str, nine_code: &str) -> bool {
    let five_code: HashSet<char> = input.chars().collect();
    let six_code: HashSet<char> = six_code.chars().collect();
    let nine_code: HashSet<char> = nine_code.chars().collect();

    five_code.len() == 5
        && six_code.intersection(&five_code).count() == 5
        && nine_code.intersection(&five_code).count() == 5
}

pub fn is_6(input: &str, one_code: &str) -> bool {
    let six_code: HashSet<char> = input.chars().collect();

    six_code.len() == 6 && six_code.intersection(&one_code.chars().collect()).count() == 1
}

pub fn is_7(input: &str) -> bool {
    let seven_code: HashSet<char> = input.chars().collect();

    seven_code.len() == 3
}

pub fn is_8(input: &str) -> bool {
    let eight_code: HashSet<char> = input.chars().collect();

    eight_code.len() == 7
}

pub fn is_9(input: &str, four_code: &str) -> bool {
    let nine_code: HashSet<char> = input.chars().collect();

    nine_code.len() == 6 && nine_code.intersection(&four_code.chars().collect()).count() == 4
}

pub fn decode(data: Vec<String>) -> Vec<String> {
    let mut decode = vec![
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    ];

    // Get unique codes first
    for d in &data {
        if is_1(d) {
            decode[1] = d.clone();
        } else if is_4(d) {
            decode[4] = d.clone();
        } else if is_7(d) {
            decode[7] = d.clone();
        } else if is_8(d) {
            decode[8] = d.clone();
        }
    }

    // Then attempt 2nd order codes
    for d in &data {
        if is_6(d, &decode[1]) {
            decode[6] = d.clone();
        } else if is_9(d, &decode[4]) {
            decode[9] = d.clone();
        }
    }

    // Then solve ugly codes
    for d in &data {
        if is_0(d, &decode[6], &decode[9]) {
            decode[0] = d.clone();
        } else if is_3(d, &decode[1], &decode[8]) {
            decode[3] = d.clone();
        } else if is_5(d, &decode[6], &decode[9]) {
            decode[5] = d.clone();
        }
    }

    // Then solve the really ugly one
    let inputs: HashSet<&String> = data.iter().collect();
    let found: HashSet<&String> = decode.iter().collect();
    decode[2] = inputs.difference(&found).next().unwrap().to_string();

    decode
}

fn get_digit(decode: &[String], coded_digit: &str) -> i32 {
    let coded_digit: HashSet<char> = coded_digit.chars().collect();

    for (i, x) in decode.iter().enumerate() {
        let key: HashSet<char> = x.chars().collect();
        if key == coded_digit {
            return i as i32;
        }
    }
    panic!("badness")
}

pub fn decode_score(decode: Vec<String>, coded_score: Vec<String>) -> i32 {
    get_digit(&decode, &coded_score[0]) * 1000
        + get_digit(&decode, &coded_score[1]) * 100
        + get_digit(&decode, &coded_score[2]) * 10
        + get_digit(&decode, &coded_score[3])
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example1() {
        let data = read_input("example.txt");

        let mut count = 0;
        for (_, values) in data {
            for value in values {
                if maybe1478(&value) {
                    count += 1;
                }
            }
        }
        assert_eq!(count, 26);
    }

    #[test]
    fn part1() {
        let data = read_input("input.txt");

        let mut count = 0;
        for (_, values) in data {
            for value in values {
                if maybe1478(&value) {
                    count += 1;
                }
            }
        }
        assert_eq!(count, 397);
    }

    #[test]
    fn example2a() {
        let data = vec![
            "acedgfb".to_string(),
            "cdfbe".to_string(),
            "gcdfa".to_string(),
            "fbcad".to_string(),
            "dab".to_string(),
            "cefabd".to_string(),
            "cdfgeb".to_string(),
            "eafb".to_string(),
            "cagedb".to_string(),
            "ab".to_string(),
        ];

        let coded_score = vec![
            "cdfeb".to_string(),
            "fcadb".to_string(),
            "cdfeb".to_string(),
            "cdbaf".to_string(),
        ];

        let decode = decode(data);

        let decoded_score = decode_score(decode, coded_score);
        assert_eq!(decoded_score, 5353);
    }

    #[test]
    fn example2b() {
        let input = read_input("example.txt");
        let mut sum = 0;

        for (data, score) in input {
            let decode = decode(data);
            sum += decode_score(decode, score);
        }
        assert_eq!(sum, 61229)
    }

    #[test]
    fn part2() {
        let input = read_input("input.txt");
        let mut sum = 0;

        for (data, score) in input {
            let decode = decode(data);
            sum += decode_score(decode, score);
        }
        assert_eq!(sum, 1027422)
    }
}
