use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(filename: &str) -> Vec<Vec<i32>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut elves = Vec::new();

    let mut elf = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            elves.push(elf);
            elf = Vec::new();
        } else {
            elf.push(line.parse::<i32>().unwrap());
        }
    }
    elves.push(elf);

    elves
}

pub fn find_top(bags: Vec<Vec<i32>>) -> i32 {
    let mut values: Vec<i32> = bags.iter().map(|bag| bag.iter().sum()).collect();
    values.sort_by(|a, b| b.cmp(a));

    values[0]
}
pub fn find_top3(bags: Vec<Vec<i32>>) -> i32 {
    let mut values: Vec<i32> = bags.iter().map(|bag| bag.iter().sum()).collect();
    values.sort_by(|a, b| b.cmp(a));

    values[0] + values[1] + values[2]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let data = read_input("example1.txt");
        assert_eq!(find_top(data), 24000);
    }

    #[test]
    fn part1() {
        let data = read_input("input.txt");
        assert_eq!(find_top(data), 72718);
    }

    #[test]
    fn example2() {
        let data = read_input("example1.txt");
        assert_eq!(find_top3(data), 45000);
    }

    #[test]
    fn part2() {
        let data = read_input("input.txt");
        assert_eq!(find_top3(data), 213089);
    }
}
