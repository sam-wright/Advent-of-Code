use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

type EnergyGrid = HashMap<(usize, usize), u32>;

pub fn read_input(filename: &str) -> EnergyGrid {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut grid = EnergyGrid::new();

    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            grid.insert((x, y), c.to_digit(10).unwrap());
        }
    }
    grid
}

pub fn print_grid(grid: &EnergyGrid) {
    for y in 0..100 {
        for x in 0..100 {
            let v = grid.get(&(x, y));
            match v {
                Some(v) => print!("{}", v),
                None => {
                    println!();
                    if x == 0 {
                        return;
                    }
                    break;
                }
            }
        }
    }
    println!();
}

fn increment_grid(grid: &EnergyGrid) -> EnergyGrid {
    let mut new_grid = EnergyGrid::new();

    for (&k, &v) in grid {
        new_grid.insert(k, v + 1);
    }

    new_grid
}

fn increment_cell(grid: &mut EnergyGrid, location: (usize, usize)) {
    if grid.contains_key(&location) {
        let entry = grid.entry(location).or_default();
        *entry += 1;
    }
}

fn reset_cell(grid: &mut EnergyGrid, location: (usize, usize)) {
    if grid.contains_key(&location) {
        let entry = grid.entry(location).or_default();
        *entry = 0;
    }
}

fn get_flashes(grid: &EnergyGrid) -> HashSet<(usize, usize)> {
    let mut flashes = HashSet::new();
    for (location, &v) in grid {
        if v > 9 {
            flashes.insert(*location);
        }
    }
    flashes
}

pub fn flash_grid(grid: &EnergyGrid) -> (EnergyGrid, usize) {
    let mut new_grid = grid.clone();
    let mut flashed = HashSet::new();
    let mut flashes = get_flashes(&new_grid);

    while flashes.difference(&flashed).count() != 0 {
        for (x, y) in flashes.difference(&flashed) {
            increment_cell(&mut new_grid, (x + 1, *y));
            increment_cell(&mut new_grid, (x + 1, y + 1));

            increment_cell(&mut new_grid, (*x, *y));
            increment_cell(&mut new_grid, (*x, y + 1));

            if *y > 0 {
                increment_cell(&mut new_grid, (x + 1, y - 1));
                increment_cell(&mut new_grid, (*x, y - 1));
            }
            if *x > 0 {
                increment_cell(&mut new_grid, (x - 1, *y));
                increment_cell(&mut new_grid, (x - 1, y + 1));
            }

            if *x > 0 && *y > 0 {
                increment_cell(&mut new_grid, (x - 1, y - 1));
            }
        }
        flashed = flashes.clone();
        flashes = get_flashes(&new_grid);
    }

    for location in &flashed {
        reset_cell(&mut new_grid, *location);
    }

    (new_grid, flashed.len())
}

fn check_for_sync(grid: &EnergyGrid) -> bool {
    for &v in grid.values() {
        if v != 0 {
            return false;
        }
    }
    true
}

pub fn step(grid: &EnergyGrid, times: usize) -> usize {
    let mut grid = grid.clone();
    // println!("Start: (score: 0)");
    // print_grid(&grid);

    let mut total = 0;
    for _iter in 0..times {
        grid = increment_grid(&grid);

        let (new_grid, score) = flash_grid(&grid);

        total += score;
        grid = new_grid;

        // println!("After step: {} (score: {})", _iter + 1, total);
        // print_grid(&grid);
    }
    total
}

pub fn find_sync(grid: &EnergyGrid) -> usize {
    let mut grid = grid.clone();
    // println!("Start: (score: 0)");
    // print_grid(&grid);

    let mut steps = 0;
    loop {
        grid = increment_grid(&grid);

        let (new_grid, _) = flash_grid(&grid);

        steps += 1;
        grid = new_grid;
        if check_for_sync(&grid) {
            break;
        }
        // print_grid(&grid);
    }
    steps
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn example1a() {
        let energies = read_input("example1.txt");
        let thing = step(&energies, 2);
        assert_eq!(thing, 9);
    }

    #[test]
    fn example1b() {
        let energies = read_input("example2.txt");
        let after_10 = step(&energies, 10);
        assert_eq!(after_10, 204);

        let after_100 = step(&energies, 100);
        assert_eq!(after_100, 1656);
    }

    #[test]
    fn part1() {
        let energies = read_input("input.txt");
        let after_100 = step(&energies, 100);
        assert_eq!(after_100, 1688);
    }

    #[test]
    fn example2() {
        let energies = read_input("example2.txt");
        let sync = find_sync(&energies);
        assert_eq!(sync, 195);
    }

    #[test]
    fn part2() {
        let energies = read_input("input.txt");
        let sync = find_sync(&energies);
        assert_eq!(sync, 403);
    }
}
