use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(filename: &str) -> Vec<(String, String)> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut rucksacks = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let a = line.split_at(line.len() / 2);

        rucksacks.push((a.0.to_owned(), a.1.to_owned()));
        assert!(a.0.len() == a.1.len());
    }

    rucksacks
}

pub fn score_rucksacks(rucksacks: &Vec<(String, String)>) -> i32 {
    let mut score = 0;
    for rucksack in rucksacks {
        let left: HashSet<char> = rucksack.0.chars().collect();
        let right: HashSet<char> = rucksack.1.chars().collect();
        let common: Vec<&char> = left.intersection(&right).collect();

        assert!(common.len() == 1);

        let common = *common[0];

        if common.is_ascii_lowercase() {
            score += common as i32 - 96;
        } else {
            score += common as i32 - 38;
        }
    }
    score
}

pub fn score_rucksack_groups(rucksacks: &[(String, String)]) -> i32 {
    let mut score = 0;
    for group in rucksacks.chunks(3) {
        let mut elf1 = HashSet::new();
        let mut elf2 = HashSet::new();
        let mut elf3 = HashSet::new();
        elf1.extend(group[0].0.chars());
        elf1.extend(group[0].1.chars());
        elf2.extend(group[1].0.chars());
        elf2.extend(group[1].1.chars());
        elf3.extend(group[2].0.chars());
        elf3.extend(group[2].1.chars());

        let intersection: Vec<&char> = elf1
            .iter()
            .filter(|k| elf2.contains(k))
            .filter(|k| elf3.contains(k))
            .collect();

        assert!(intersection.len() == 1);

        let intersection = *intersection[0];

        if intersection.is_ascii_lowercase() {
            score += intersection as i32 - 96;
        } else {
            score += intersection as i32 - 38;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabet() {
        assert_eq!('A' as u32 - 38, 27);
        assert_eq!('Z' as u32 - 38, 52);

        assert_eq!('a' as u32 - 96, 1);
        assert_eq!('z' as u32 - 96, 26);
    }

    #[test]
    fn example1() {
        let rucksacks = read_input("example1.txt");

        score_rucksacks(&rucksacks);
        assert_eq!(score_rucksacks(&rucksacks), 157);
    }

    #[test]
    fn part1() {
        let rucksacks = read_input("input.txt");
        assert_eq!(score_rucksacks(&rucksacks), 7766);
    }

    #[test]
    fn example2() {
        let rucksacks = read_input("example1.txt");
        assert_eq!(score_rucksack_groups(&rucksacks), 70);
    }

    #[test]
    fn part2() {
        let rucksacks = read_input("input.txt");
        assert_eq!(score_rucksack_groups(&rucksacks), 2415);
    }
}
