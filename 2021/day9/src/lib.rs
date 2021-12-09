use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(filename: &str) -> Vec<Vec<u32>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

pub fn is_low(depth_map: &[Vec<u32>], x: usize, y: usize) -> bool {
    let max_x = depth_map[0].len() - 1;
    let max_y = depth_map.len() - 1;

    let point = depth_map[y][x];

    if x > 0 && depth_map[y][x - 1] <= point {
        return false;
    }

    if x < max_x && depth_map[y][x + 1] <= point {
        return false;
    }

    if y > 0 && depth_map[y - 1][x] <= point {
        return false;
    }

    if y < max_y && depth_map[y + 1][x] <= point {
        return false;
    }
    true
}

pub fn calculate_risk(depth_map: &[Vec<u32>]) -> u32 {
    let mut total_risk = 0;
    let width = depth_map[0].len();
    let height = depth_map.len();

    for x in 0..width {
        for y in 0..height {
            if is_low(depth_map, x, y) {
                total_risk += 1 + depth_map[y][x];
            }
        }
    }

    total_risk
}

pub fn basin_area(depth_map: &[Vec<u32>], x: usize, y: usize) -> u32 {
    let max_x = depth_map[0].len() - 1;
    let max_y = depth_map.len() - 1;

    let mut area = 0;
    let mut explored = HashSet::new();
    let mut queue = VecDeque::new();

    // seed the search
    queue.push_back((x, y));

    while !queue.is_empty() {
        // pop new point
        let (x, y) = queue.pop_front().unwrap();

        // mark it explored
        if explored.contains(&(x, y)) {
            continue;
        } else {
            explored.insert((x, y));
        }

        // accumulate
        area += 1;

        // expand
        if x < max_x && depth_map[y][x + 1] > depth_map[y][x] && depth_map[y][x + 1] != 9 {
            queue.push_back((x + 1, y));
        }
        if x > 0 && depth_map[y][x - 1] > depth_map[y][x] && depth_map[y][x - 1] != 9 {
            queue.push_back((x - 1, y));
        }
        if y < max_y && depth_map[y + 1][x] > depth_map[y][x] && depth_map[y + 1][x] != 9 {
            queue.push_back((x, y + 1));
        }
        if y > 0 && depth_map[y - 1][x] > depth_map[y][x] && depth_map[y - 1][x] != 9 {
            queue.push_back((x, y - 1));
        }
    }

    area
}

pub fn find_basins(depth_map: &[Vec<u32>]) -> u32 {
    let mut basins = Vec::new();
    let width = depth_map[0].len();
    let height = depth_map.len();

    for x in 0..width {
        for y in 0..height {
            if is_low(depth_map, x, y) {
                basins.push(basin_area(depth_map, x, y));
            }
        }
    }
    basins.sort_unstable();

    basins.iter().rev().next().unwrap()
        * basins.iter().rev().nth(1).unwrap()
        * basins.iter().rev().nth(2).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example1() {
        let depth_map = read_input("example.txt");

        let this = is_low(&depth_map, 1, 0);
        assert!(this); // must be true

        let that = is_low(&depth_map, 2, 2);
        assert!(that); // must be true

        let other = is_low(&depth_map, 9, 0);
        assert!(other); // must be true

        let again = is_low(&depth_map, 6, 4);
        assert!(again); // must be true

        let risk = calculate_risk(&depth_map);
        assert_eq!(risk, 15);
    }
    #[test]
    fn part1() {
        let depth_map = read_input("input.txt");

        let risk = calculate_risk(&depth_map);
        assert_eq!(risk, 560);
    }

    #[test]
    fn example2() {
        let depth_map = read_input("example.txt");

        let this = basin_area(&depth_map, 1, 0);
        assert_eq!(this, 3);

        let that = basin_area(&depth_map, 9, 0);
        assert_eq!(that, 9);

        let other = basin_area(&depth_map, 6, 4);
        assert_eq!(other, 9);

        let again = basin_area(&depth_map, 2, 2);
        assert_eq!(again, 14);

        let basins = find_basins(&depth_map);
        assert_eq!(basins, 1134);
    }

    #[test]
    fn part2() {
        let depth_map = read_input("input.txt");

        let basins = find_basins(&depth_map);
        assert_eq!(basins, 959136);
    }
}
