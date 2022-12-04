use std::{
    fs::File,
    io::{BufRead, BufReader},
};
extern crate regex;
use regex::{Captures, Regex};

pub fn full_overlap(caps: &Captures) -> bool {
    ((caps[1].parse::<i32>().unwrap() >= caps[3].parse::<i32>().unwrap())
        && (caps[4].parse::<i32>().unwrap() >= caps[2].parse::<i32>().unwrap()))
        || ((caps[1].parse::<i32>().unwrap() <= caps[3].parse::<i32>().unwrap())
            && (caps[4].parse::<i32>().unwrap() <= caps[2].parse::<i32>().unwrap()))
}

pub fn partial_overlap(caps: &Captures) -> bool {
    (caps[3].parse::<i32>().unwrap() >= caps[1].parse::<i32>().unwrap())
        && (caps[3].parse::<i32>().unwrap() <= caps[2].parse::<i32>().unwrap())
        || (caps[4].parse::<i32>().unwrap() >= caps[1].parse::<i32>().unwrap())
            && (caps[4].parse::<i32>().unwrap() <= caps[2].parse::<i32>().unwrap())
}

pub fn check_overlaps(filename: &str, pred: fn(&Captures) -> bool) -> i32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let caps = re.captures(&line).unwrap();

        if pred(&caps) {
            score += 1;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(check_overlaps("example1.txt", full_overlap), 2);
    }

    #[test]
    fn part1() {
        assert_eq!(check_overlaps("input.txt", full_overlap), 507);
    }

    #[test]
    fn example2() {
        assert_eq!(
            check_overlaps("example1.txt", |c| {
                partial_overlap(&c) || full_overlap(&c)
            }),
            4
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            check_overlaps("input.txt", |c| { partial_overlap(&c) || full_overlap(&c) }),
            897
        );
    }
}
