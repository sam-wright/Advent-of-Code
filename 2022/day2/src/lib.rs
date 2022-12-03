use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug)]
enum Goal {
    Win,
    Draw,
    Lose,
}

#[derive(Debug)]
pub struct Round {
    opponent: Choice,
    you: Choice,
}

use crate::Choice::*;
use crate::Goal::*;

pub fn read_input(filename: &str) -> Vec<Round> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut rounds = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let moves: Vec<&str> = line.split(' ').collect();
        let opponent = match moves[0] {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissor,
            _ => panic!("arst"),
        };
        let you = match moves[1] {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissor,
            _ => panic!("arst"),
        };
        rounds.push(Round { opponent, you });
    }

    rounds
}

pub fn correct_input(filename: &str) -> Vec<Round> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut rounds = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let moves: Vec<&str> = line.split(' ').collect();
        let opponent = match moves[0] {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissor,
            _ => panic!("arst"),
        };
        let goal = match moves[1] {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("arst"),
        };
        let you = match (goal, &opponent) {
            (Lose, Rock) => Scissor,
            (Lose, Paper) => Rock,
            (Lose, Scissor) => Paper,
            (Draw, Rock) => Rock,
            (Draw, Paper) => Paper,
            (Draw, Scissor) => Scissor,
            (Win, Rock) => Paper,
            (Win, Paper) => Scissor,
            (Win, Scissor) => Rock,
        };

        rounds.push(Round { opponent, you });
    }

    rounds
}

pub fn score_game(rounds: &[Round]) -> i32 {
    let mut score = 0;
    for round in rounds {
        score += match &round.you {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        };
        score += match (&round.you, &round.opponent) {
            (Rock, Rock) => 3,
            (Rock, Paper) => 0,
            (Rock, Scissor) => 6,
            (Paper, Rock) => 6,
            (Paper, Paper) => 3,
            (Paper, Scissor) => 0,
            (Scissor, Rock) => 0,
            (Scissor, Paper) => 6,
            (Scissor, Scissor) => 3,
        };
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let rounds = read_input("example1.txt");
        assert_eq!(score_game(&rounds), 15)
    }

    #[test]
    fn part1() {
        let rounds = read_input("input.txt");
        assert_eq!(score_game(&rounds), 14069)
    }

    #[test]
    fn example2() {
        let rounds = correct_input("example1.txt");
        assert_eq!(score_game(&rounds), 12)
    }

    #[test]
    fn part2() {
        let rounds = correct_input("input.txt");
        assert_eq!(score_game(&rounds), 12411) //13081  too high
    }
}
