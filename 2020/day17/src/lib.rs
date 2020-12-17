use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

type Grid = HashMap<(i32, i32, i32, i32), Cell>;
#[derive(Debug, PartialEq)]
pub enum Cell {
    Inactive,
    Active,
}

pub fn read_input(filename: &str) -> Vec<String> {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let collection: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    collection
}

// Because this algorithm never expands its search space, we can use the same
// functions and differentiate part-1/part-2 by simply initializing a 3D/4D
// search space
fn empty_grid_3d() -> Grid {
    let mut grid = HashMap::new();
    let sz = 13;
    for x in -sz..sz {
        for y in -sz..sz {
            for z in -sz..sz {
                grid.insert((x as i32, y as i32, z as i32, 0), Cell::Inactive);
            }
        }
    }
    grid
}

fn empty_grid_4d() -> Grid {
    let mut grid = HashMap::new();
    let sz = 13;
    for x in -sz..sz {
        for y in -sz..sz {
            for z in -sz..sz {
                for w in -sz..sz {
                    grid.insert((x as i32, y as i32, z as i32, w), Cell::Inactive);
                }
            }
        }
    }
    grid
}

pub fn input_to_grid_3d(input: &Vec<String>) -> Grid {
    let mut grid = empty_grid_3d();

    let z = 0;
    let w = 0;
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let cell = if c == '#' {
                Cell::Active
            } else {
                Cell::Inactive
            };
            grid.insert((x as i32, y as i32, z, w), cell);
        }
    }

    grid
}

pub fn input_to_grid_4d(input: &Vec<String>) -> Grid {
    let mut grid = empty_grid_4d();

    let z = 0;
    let w = 0;
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let cell = if c == '#' {
                Cell::Active
            } else {
                Cell::Inactive
            };
            grid.insert((x as i32, y as i32, z, w), cell);
        }
    }

    grid
}

pub fn count_active(grid: &Grid) -> i32 {
    grid.iter().fold(
        0,
        |acc, (_, x)| {
            if x == &Cell::Active {
                acc + 1
            } else {
                acc
            }
        },
    )
}

fn is_active(grid: &Grid, location: &(i32, i32, i32, i32)) -> i32 {
    if grid.contains_key(location) && grid[location] == Cell::Active {
        1
    } else {
        0
    }
}

pub fn count_active_neighbors(grid: &Grid, location: &(i32, i32, i32, i32)) -> i32 {
    let (x, y, z, w) = *location;

    let mut count = 0;

    // Beautiful in its sillyness isnt it?
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    if (dx, dy, dz, dw) == (0, 0, 0, 0) {
                        continue;
                    }
                    count += is_active(&grid, &(x + dx, y + dy, z + dz, w + dw));
                }
            }
        }
    }
    count
}

pub fn cycle(grid: &Grid) -> Grid {
    let mut new_grid = HashMap::new();

    /*
    If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
    If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
    */

    // Evaluate
    for ((x, y, z, w), cell) in grid {
        let neighbors = count_active_neighbors(&grid, &(*x, *y, *z, *w));
        let new_cell = if cell == &Cell::Active && (neighbors == 2 || neighbors == 3) {
            Cell::Active
        } else if cell == &Cell::Inactive && neighbors == 3 {
            Cell::Active
        } else {
            Cell::Inactive
        };

        new_grid.insert((*x, *y, *z, *w), new_cell);
    }
    println!("Cycle Complete");
    new_grid
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_example() {
        let input = read_input("example1.txt");
        let grid = input_to_grid_3d(&input);
        assert_eq!(count_active(&grid), 5); //0-cycles
        assert_eq!(count_active_neighbors(&grid, &(1, 1, 0, 0)), 5);
        assert_eq!(count_active_neighbors(&grid, &(2, 0, 0, 0)), 2);
        assert_eq!(count_active_neighbors(&grid, &(0, 0, 0, 0)), 1);

        let grid = cycle(&grid);

        assert_eq!(count_active(&grid), 11); // 1-cycles

        let grid = cycle(&grid);
        assert_eq!(count_active(&grid), 21); //2-cycles

        let grid = cycle(&grid);
        assert_eq!(count_active(&grid), 38); //3-cycles

        let grid = cycle(&grid);
        let grid = cycle(&grid);
        let grid = cycle(&grid);
        assert_eq!(count_active(&grid), 112); //6-cycles
    }

    #[test]
    fn part_1_solution() {
        let input = read_input("input.txt");
        let grid = input_to_grid_3d(&input);

        let grid = cycle(&grid); //1
        let grid = cycle(&grid); //2
        let grid = cycle(&grid); //3
        let grid = cycle(&grid); //4
        let grid = cycle(&grid); //5
        let grid = cycle(&grid); //6
        assert_eq!(count_active(&grid), 426);
        // guessed 360 (too low)
    }

    #[test]
    fn part_2_example() {
        let input = read_input("example1.txt");
        let grid = input_to_grid_4d(&input);

        let grid = cycle(&grid);

        assert_eq!(count_active(&grid), 29); // 1-cycles

        let grid = cycle(&grid);
        let grid = cycle(&grid);
        let grid = cycle(&grid);
        let grid = cycle(&grid);
        let grid = cycle(&grid);
        assert_eq!(count_active(&grid), 848); //6-cycles
    }

    #[test]
    fn part_2_solution() {
        let input = read_input("input.txt");
        let grid = input_to_grid_4d(&input);

        let grid = cycle(&grid);
        let grid = cycle(&grid);
        let grid = cycle(&grid);
        let grid = cycle(&grid);
        let grid = cycle(&grid);
        let grid = cycle(&grid);
        assert_eq!(count_active(&grid), 1892); //6-cycles
    }
}
