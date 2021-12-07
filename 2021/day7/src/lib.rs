use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn read_input(filename: &str) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut data = String::new();

    reader.read_to_string(&mut data).unwrap();
    data.split(',').map(|x| x.parse::<i32>().unwrap()).collect()
}

pub fn align_to(positions: &[i32], desired_position: i32) -> i32 {
    positions
        .iter()
        .fold(0, |cost, x| cost + (x - desired_position).abs())
}

pub fn crab_align_to(positions: &[i32], desired_position: i32) -> i32 {
    //
    // simply observe that the cost function follows the triangle series
    // 0, 1, 3, 6, 10, 15, ...
    //
    positions.iter().fold(0, |cost, x| {
        let f = (x - desired_position).abs();
        cost + f * (f + 1) / 2
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example1() {
        let positions = read_input("example.txt");

        assert_eq!(align_to(&positions, 2), 37); // given solutions
        assert_eq!(align_to(&positions, 1), 41); // given solutions
        assert_eq!(align_to(&positions, 3), 39); // given solutions
        assert_eq!(align_to(&positions, 10), 71); // given solutions

        let mut min_target = i32::MIN;
        let mut min_cost = i32::MAX;

        for x in *positions.iter().min().unwrap()..*positions.iter().max().unwrap() {
            let cost = align_to(&positions, x);
            if cost < min_cost {
                min_cost = cost;
                min_target = x;
            }
        }
        assert_eq!(min_target, 2);
        assert_eq!(min_cost, 37);
    }

    #[test]
    fn part1() {
        let positions = read_input("input.txt");

        let mut min_target = i32::MIN;
        let mut min_cost = i32::MAX;

        for x in *positions.iter().min().unwrap()..*positions.iter().max().unwrap() {
            let cost = align_to(&positions, x);
            if cost < min_cost {
                min_cost = cost;
                min_target = x;
            }
        }
        assert_eq!(min_target, 337);
        assert_eq!(min_cost, 342641);
    }

    #[test]
    fn example2() {
        let positions = read_input("example.txt");

        assert_eq!(crab_align_to(&positions, 5), 168); // given solutions
        assert_eq!(crab_align_to(&positions, 2), 206); // given solutions

        let mut min_target = i32::MIN;
        let mut min_cost = i32::MAX;

        for x in *positions.iter().min().unwrap()..*positions.iter().max().unwrap() {
            let cost = crab_align_to(&positions, x);
            if cost < min_cost {
                min_cost = cost;
                min_target = x;
            }
        }
        assert_eq!(min_target, 5);
        assert_eq!(min_cost, 168);
    }

    #[test]
    fn part2() {
        let positions = read_input("input.txt");

        let mut min_target = i32::MIN;
        let mut min_cost = i32::MAX;

        for x in *positions.iter().min().unwrap()..*positions.iter().max().unwrap() {
            let cost = crab_align_to(&positions, x);
            if cost < min_cost {
                min_cost = cost;
                min_target = x;
            }
        }
        assert_eq!(min_target, 470);
        assert_eq!(min_cost, 93006301);
    }
}
