use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// Parses input and returns relative motion
pub fn read_input(filename: &str) -> Vec<(i32 /*Horizontal*/, i32 /*Depth*/)> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let words = line.unwrap();
            let mut i = words.split(' ').into_iter();
            let keyword = i.next().unwrap();
            let command = i.next().unwrap().parse::<i32>().unwrap();

            match keyword {
                "forward" => (command, 0),
                "down" => (0, command),
                "up" => (0, -command),
                _ => panic!("invalid input!"),
            }
        })
        .collect()
}

// Parses input and returns relative motion using aim definition
pub fn read_input_with_aim(filename: &str) -> Vec<(i32 /*Horizontal*/, i32 /*Depth*/)> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut aim = 0;
    reader
        .lines()
        .map(|line| {
            let words = line.unwrap();
            let mut i = words.split(' ').into_iter();
            let keyword = i.next().unwrap();
            let command = i.next().unwrap().parse::<i32>().unwrap();
            match keyword {
                "forward" => (command, command * aim),
                "down" => {
                    aim += command;
                    (0, 0)
                }
                "up" => {
                    aim -= command;
                    (0, 0)
                }
                _ => panic!("invalid input!"),
            }
        })
        .collect()
}

pub fn find_displacement(input: &Vec<(i32, i32)>) -> (i32, i32) {
    input
        .iter()
        .fold((0, 0), |dist, coord| (dist.0 + coord.0, dist.1 + coord.1))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let data = read_input("example1.txt");
        let displacement = find_displacement(&data);

        assert_eq!(displacement.0 * displacement.1, 150);
    }

    #[test]
    fn part1() {
        let data = read_input("input.txt");
        let displacement = find_displacement(&data);
        assert_eq!(displacement.0 * displacement.1, 1692075);
    }

    #[test]
    fn example2() {
        let data = read_input_with_aim("example1.txt");
        let displacement = find_displacement(&data);
        assert_eq!(displacement.0 * displacement.1, 900);
    }

    #[test]
    fn part2() {
        let data = read_input_with_aim("input.txt");
        let displacement = find_displacement(&data);

        assert_eq!(displacement.0 * displacement.1, 1749524700);
    }
}
