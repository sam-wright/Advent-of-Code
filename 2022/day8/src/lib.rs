use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

type Map = HashMap<(usize, usize), u32>;
pub fn read_input(filename: &str) -> (Map, usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut map = HashMap::new();
    let mut len_y = 0;
    let mut len_x = 0;
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (x, z) in line.chars().enumerate() {
            map.insert((x, y), z.to_digit(10).unwrap());
            len_x = x;
        }
        len_y = y;
    }

    (map, len_x + 1, len_y + 1)
}

pub fn count_visibility(map: &Map, len_x: usize, len_y: usize) -> usize {
    let mut count = (len_x + len_y - 2) * 2;

    for x in 1..len_x - 1 {
        'out: for y in 1..len_y - 1 {
            let tree = map[&(x, y)];

            // check visibility
            // check up
            'up: {
                for dy in 0..=y - 1 {
                    if tree > map[&(x, dy)] {
                        continue;
                    }
                    break 'up;
                }
                count += 1;
                continue 'out;
            }

            // check down
            'down: {
                for dy in y + 1..len_y {
                    if tree > map[&(x, dy)] {
                        continue;
                    }
                    break 'down;
                }
                count += 1;
                continue 'out;
            }

            // check left
            'left: {
                for dx in 0..=x - 1 {
                    if tree > map[&(dx, y)] {
                        continue;
                    }
                    break 'left;
                }
                count += 1;
                continue 'out;
            }

            // check right
            'right: {
                for dx in x + 1..len_x {
                    if tree > map[&(dx, y)] {
                        continue;
                    }
                    break 'right;
                }
                count += 1;
                continue 'out;
            }
        }
    }
    count
}

pub fn score_visibility(map: &Map, len_x: usize, len_y: usize, x: usize, y: usize) -> usize {
    let tree = map[&(x, y)];

    // check visibility
    let up = {
        let mut score = 0;
        for dy in (0..=(y - 1)).rev() {
            score += 1;
            if tree > map[&(x, dy)] {
                continue;
            }
            break;
        }
        score
    };

    let down = {
        let mut score = 0;
        for dy in (y + 1)..=len_y - 1 {
            score += 1;
            if tree > map[&(x, dy)] {
                continue;
            }
            break;
        }
        score
    };

    let left = {
        let mut score = 0;
        // for dx in (x - 1)..=0 {
        for dx in (0..=(x - 1)).rev() {
            score += 1;
            if tree > map[&(dx, y)] {
                continue;
            }
            break;
        }
        score
    };

    let right = {
        let mut score = 0;
        for dx in (x + 1)..=len_x - 1 {
            score += 1;
            if tree > map[&(dx, y)] {
                continue;
            }
            break;
        }
        score
    };
    right * left * up * down
}

pub fn find_best(map: &Map, len_x: usize, len_y: usize) -> usize {
    let mut best = 0;
    for x in 1..len_x - 1 {
        for y in 1..len_y - 1 {
            let score = score_visibility(map, len_x, len_y, x, y);

            if score > best {
                best = score;
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let (map, lx, ly) = read_input("example1.txt");
        assert_eq!(count_visibility(&map, lx, ly), 21);
    }

    #[test]
    fn part1() {
        let (map, lx, ly) = read_input("input.txt");
        assert_eq!(count_visibility(&map, lx, ly), 1854);
    }

    #[test]
    fn example2() {
        let (map, lx, ly) = read_input("example1.txt");
        assert_eq!(score_visibility(&map, lx, ly, 2, 1), 4);
        assert_eq!(score_visibility(&map, lx, ly, 2, 3), 8);
        assert_eq!(find_best(&map, lx, ly), 8);
    }

    #[test]
    fn part2() {
        let (map, lx, ly) = read_input("input.txt");
        assert_eq!(find_best(&map, lx, ly), 527340);
    }
}
