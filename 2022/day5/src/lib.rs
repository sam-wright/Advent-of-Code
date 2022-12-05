use std::{
    fs::File,
    io::{BufRead, BufReader},
};

extern crate regex;
use regex::Regex;

fn do_move(from: usize, to: usize, mut stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let v = stacks[from - 1].pop().unwrap();
    stacks[to - 1].push(v);

    stacks
}

pub fn read_input_9000(filename: &str, start: usize, mut stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for (idx, line) in reader.lines().enumerate() {
        if idx >= start {
            let line = line.unwrap();
            let caps = re.captures(&line).unwrap();
            assert!(caps.len() == 4);

            let from = caps[2].parse::<usize>().unwrap();
            let to = caps[3].parse::<usize>().unwrap();
            let times = caps[1].parse::<usize>().unwrap();

            for _ in 0..times {
                stacks = do_move(from, to, stacks);
            }
        }
    }
    stacks
}

pub fn read_input_9001(filename: &str, start: usize, mut stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for (idx, line) in reader.lines().enumerate() {
        if idx >= start {
            let line = line.unwrap();
            let caps = re.captures(&line).unwrap();
            assert!(caps.len() == 4);

            let from = caps[2].parse::<usize>().unwrap();
            let to = caps[3].parse::<usize>().unwrap();
            let times = caps[1].parse::<usize>().unwrap();

            let l = stacks[from - 1].len() - times;
            for m in stacks[from - 1].split_off(l) {
                stacks[to - 1].push(m);
            }
        }
    }
    stacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let stacks = vec![vec!['z', 'n'], vec!['m', 'c', 'd'], vec!['p']];
        let stacks = read_input_9000("example1.txt", 5, stacks);

        println!();

        let mut ans = String::new();
        for s in stacks {
            ans.push(*s.last().unwrap());
        }
        assert_eq!(ans.to_uppercase(), "CMZ");
    }

    #[test]
    fn part1() {
        let stacks = vec![
            vec!['s', 'c', 'v', 'n'],
            vec!['z', 'm', 'j', 'h', 'n', 's'],
            vec!['M', 'C', 'T', 'G', 'J', 'N', 'D'],
            vec!['T', 'D', 'F', 'J', 'W', 'R', 'M'],
            vec!['P', 'F', 'H'],
            vec!['C', 'T', 'Z', 'H', 'J'],
            vec!['D', 'P', 'R', 'Q', 'F', 'S', 'L', 'Z'],
            vec!['C', 'S', 'L', 'H', 'D', 'F', 'P', 'W'],
            vec!['D', 'S', 'M', 'P', 'F', 'N', 'G', 'Z'],
        ];
        let stacks = read_input_9000("input.txt", 10, stacks);

        let mut ans = String::new();
        for s in stacks {
            ans.push(*s.last().unwrap());
        }
        assert_eq!(ans.to_uppercase(), "CNSZFDVLJ");
    }

    #[test]
    fn example2() {
        let stacks = vec![vec!['z', 'n'], vec!['m', 'c', 'd'], vec!['p']];
        let stacks = read_input_9001("example1.txt", 5, stacks);

        println!();

        let mut ans = String::new();
        for s in stacks {
            ans.push(*s.last().unwrap());
        }
        assert_eq!(ans.to_uppercase(), "MCD");
    }

    #[test]
    fn part2() {
        let stacks = vec![
            vec!['s', 'c', 'v', 'n'],
            vec!['z', 'm', 'j', 'h', 'n', 's'],
            vec!['M', 'C', 'T', 'G', 'J', 'N', 'D'],
            vec!['T', 'D', 'F', 'J', 'W', 'R', 'M'],
            vec!['P', 'F', 'H'],
            vec!['C', 'T', 'Z', 'H', 'J'],
            vec!['D', 'P', 'R', 'Q', 'F', 'S', 'L', 'Z'],
            vec!['C', 'S', 'L', 'H', 'D', 'F', 'P', 'W'],
            vec!['D', 'S', 'M', 'P', 'F', 'N', 'G', 'Z'],
        ];
        let stacks = read_input_9001("input.txt", 10, stacks);

        let mut ans = String::new();
        for s in stacks {
            ans.push(*s.last().unwrap());
        }
        assert_eq!(ans.to_uppercase(), "QNDWLMGNS");
    }
}
