use regex::Regex;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};
extern crate regex;

#[derive(Debug)]
pub struct Move {
    dir: char,
    times: i32,
}

pub fn read_input(filename: &str) -> Vec<Move> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let re = Regex::new(r"([U|L|D|R]) (\d+)").unwrap();

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let caps = re.captures(&line).unwrap();
            Move {
                dir: caps[1].chars().next().unwrap(),
                times: caps[2].parse().unwrap(),
            }
        })
        .collect()
}

fn fix_tail(current_head: (i32, i32), current_tail: (i32, i32)) -> (i32, i32) {
    let diff = (
        (current_head.0 - current_tail.0),
        (current_head.1 - current_tail.1),
    );
    let dist = diff.0.abs() + diff.1.abs();

    // early exit (Touching)
    if dist < 2 || (dist == 2 && diff.0.abs() == diff.1.abs()) {
        return current_tail;
    }

    // If the head is ever two steps directly up, down, left, or right from the tail, the tail
    // must also move one step in that direction so it remains close enough:
    if diff == (0, 2) || diff == (0, -2) || diff == (2, 0) || diff == (-2, 0) {
        return (current_tail.0 + diff.0 / 2, current_tail.1 + diff.1 / 2);
    }

    // The only remaining case is to take a diag catch-up move
    (
        current_tail.0 + diff.0.signum(),
        current_tail.1 + diff.1.signum(),
    )
}
pub fn track_moves(moves: &Vec<Move>, n: usize) -> usize {
    let mut rope = vec![(0, 0); n];
    let mut visited = HashSet::new();

    for m in moves {
        match m.dir {
            'U' => {
                for _ in 0..m.times {
                    rope[0] = (rope[0].0, rope[0].1 + 1);
                    for nn in 1..n {
                        rope[nn] = fix_tail(rope[nn - 1], rope[nn]);
                    }
                    visited.insert(rope[n - 1]);
                }
            }
            'L' => {
                for _ in 0..m.times {
                    rope[0] = (rope[0].0 - 1, rope[0].1);

                    for nn in 1..n {
                        rope[nn] = fix_tail(rope[nn - 1], rope[nn]);
                    }
                    visited.insert(rope[n - 1]);
                }
            }
            'D' => {
                for _ in 0..m.times {
                    rope[0] = (rope[0].0, rope[0].1 - 1);

                    for nn in 1..n {
                        rope[nn] = fix_tail(rope[nn - 1], rope[nn]);
                    }
                    visited.insert(rope[n - 1]);
                }
            }
            'R' => {
                for _ in 0..m.times {
                    rope[0] = (rope[0].0 + 1, rope[0].1);

                    for nn in 1..n {
                        rope[nn] = fix_tail(rope[nn - 1], rope[nn]);
                    }
                    visited.insert(rope[n - 1]);
                }
            }
            _ => panic!("bad input"),
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let moves = read_input("example1.txt");
        assert_eq!(track_moves(&moves, 2), 13);
    }

    #[test]
    fn part1() {
        let moves = read_input("input.txt");
        assert_eq!(track_moves(&moves, 2), 6087);
    }

    #[test]
    fn example2() {
        let moves = read_input("example2.txt");
        assert_eq!(track_moves(&moves, 10), 36);
    }

    #[test]
    fn part2() {
        let moves = read_input("input.txt");
        assert_eq!(track_moves(&moves, 10), 2493);
    }
}
