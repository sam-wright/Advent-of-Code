use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn readInput(filename: &str) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect()
}

fn is_incrementing(a: i32, b: i32) -> bool {
    a < b
}

fn sliding_sum(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let data = readInput("example1.txt");

        let mut last = i32::MAX;
        let mut count = 0;
        for d in data {
            if is_incrementing(last, d) {
                count += 1;
            }
            last = d;
        }

        assert_eq!(count, 7);
    }

    #[test]
    fn part1() {
        let data = readInput("input.txt");

        let mut last = i32::MAX;
        let mut count = 0;
        for d in data {
            if is_incrementing(last, d) {
                count += 1;
            }
            last = d;
        }

        assert_eq!(count, 1393);
    }

    #[test]
    fn example2() {
        let data = readInput("example1.txt");

        let mut a = 0;
        let mut b = 0;
        let mut last = i32::MAX;
        let mut count = 0;

        for d in data {
            let sum = sliding_sum(a, b, d);
            println!("{}", sum);
            if is_incrementing(last, sum) {
                count += 1;
            }
            a = b;
            b = d;
            last = sum;
        }
        assert_eq!(count, 5)
    }

    #[test]
    fn part2() {
        let data = readInput("input.txt");

        let mut a = 0;
        let mut b = 0;
        let mut last = i32::MAX;
        let mut count = 0;

        for d in data {
            let sum = sliding_sum(a, b, d);
            println!("{}", sum);
            if is_incrementing(last, sum) {
                count += 1;
            }
            a = b;
            b = d;
            last = sum;
        }
        assert_eq!(count - 2, 1359)
    }
}
