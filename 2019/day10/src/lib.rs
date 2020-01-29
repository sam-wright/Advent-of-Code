use noisy_float::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashSet;

pub struct Map {
    pub asteroids: Vec<(usize, usize)>, // (x, y)
    pub max_x: usize,
    pub max_y: usize,
}
const PI: f64 = std::f64::consts::PI;

impl Map {
    pub fn new(input: &str) -> Self {
        let mut this_map = Self {
            asteroids: Vec::new(),
            max_x: 0,
            max_y: 0,
        };

        let lines = input.split('\n');

        for (y, line) in lines.enumerate() {
            for (x, val) in line.chars().filter(|v| *v != ' ').enumerate() {
                if val == '#' {
                    this_map.asteroids.push((x, y));
                }
                this_map.max_x = x;
            }
            this_map.max_y = y;
        }

        this_map
    }

    pub fn vaporize_asteroids(&self, position: (usize, usize)) {
        let mut relative_positions = self.get_relative_positions(position);
        dbg!(&relative_positions);

        for (_angle, distances) in &mut relative_positions {
            distances.sort_unstable();
        }

        let mut deletion = 1;
        // loop {
        for (angle, distances) in &mut relative_positions {
            let d = distances.get(0).unwrap();
            println!(
                "({})\tZapping {},{}\t({})",
                deletion,
                n64((position.0 as f32).into()) + (*d * angle.cos()).round(),
                n64((position.1 as f32).into()) + (*d * angle.sin()).round(),
                angle
            );
            deletion += 1;
        }
        // }
    }

    fn get_relative_positions(&self, position: (usize, usize)) -> BTreeMap<N64, Vec<N64>> {
        let mut angles = BTreeMap::new();
        for asteroid in &self.asteroids {
            let dx = n64((asteroid.0 as isize - position.0 as isize) as f64);
            let dy = n64((asteroid.1 as isize - position.1 as isize) as f64);

            let mut angle = dy.atan2(dx);
            if angle < -PI / 2.0 {
                angle += 2.0 * PI;
            }

            // println!("dx: {}, dy: {}, angle: {}", &dx, &dy, &angle);

            let r = (dy * dy + dx * dx).sqrt();
            if r > 0.0 {
                let e = angles.entry(angle).or_insert(Vec::new());
                e.push(r);
            }
        }
        angles
    }

    pub fn count_asteroids(&self, position: (usize, usize)) -> usize {
        let mut angles = HashSet::new();
        for asteroid in &self.asteroids {
            let dx = n64((asteroid.0 as isize - position.0 as isize) as f64);

            let dy = n64((asteroid.1 as isize - position.1 as isize) as f64);

            angles.insert(dy.atan2(dx));
        }
        angles.len()
    }

    pub fn find_best_location(&self) -> (usize, usize) {
        let mut max_asteroids = 0;
        let mut max_location = (0, 0);

        for pos in &self.asteroids {
            let score = self.count_asteroids(*pos);

            if score > max_asteroids {
                max_asteroids = score;
                max_location = *pos;
            }
        }

        max_location
    }
}

#[cfg(test)]
mod tests {
    use crate::Map;

    #[test]
    fn example2_2() {
        let grid = Map::new(
            ".#....#####...#..
            ##...##.#####..##
            ##...#...#.#####.
            ..#.....#...###..
            ..#.#.....#....##
            
            ",
        );

        let position = grid.find_best_location();
        assert_eq!(position, (8, 3));

        grid.vaporize_asteroids(position);
    }

    #[test]
    fn example2_1() {
        let grid = Map::new(
            ".#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##
            ",
        );

        let position = grid.find_best_location();
        assert_eq!(position, (11, 13));

        assert_eq!(210, grid.count_asteroids(position));

        grid.vaporize_asteroids(position);
    }

    #[test]
    fn part1_2() {
        let grid = Map::new(
            ".#......##.#..#.......#####...#..
            ...#.....##......###....#.##.....
            ..#...#....#....#............###.
            .....#......#.##......#.#..###.#.
            #.#..........##.#.#...#.##.#.#.#.
            ..#.##.#...#.......#..##.......##
            ..#....#.....#..##.#..####.#.....
            #.............#..#.........#.#...
            ........#.##..#..#..#.#.....#.#..
            .........#...#..##......###.....#
            ##.#.###..#..#.#.....#.........#.
            .#.###.##..##......#####..#..##..
            .........#.......#.#......#......
            ..#...#...#...#.#....###.#.......
            #..#.#....#...#.......#..#.#.##..
            #.....##...#.###..#..#......#..##
            ...........#...#......#..#....#..
            #.#.#......#....#..#.....##....##
            ..###...#.#.##..#...#.....#...#.#
            .......#..##.#..#.............##.
            ..###........##.#................
            ###.#..#...#......###.#........#.
            .......#....#.#.#..#..#....#..#..
            .#...#..#...#......#....#.#..#...
            #.#.........#.....#....#.#.#.....
            .#....#......##.##....#........#.
            ....#..#..#...#..##.#.#......#.#.
            ..###.##.#.....#....#.#......#...
            #.##...#............#..#.....#..#
            .#....##....##...#......#........
            ...#...##...#.......#....##.#....
            .#....#.#...#.#...##....#..##.#.#
            .#.#....##.......#.....##.##.#.##
            ",
        );

        let position = grid.find_best_location();

        assert_eq!(position, (29, 28));

        //Part-1
        assert_eq!(256, grid.count_asteroids(position));

        // Part-2
        grid.vaporize_asteroids(position);
        // guessed 39,12 (too high)
    }

    #[test]
    fn example5() {
        let grid = Map::new(
            ".#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##",
        );

        let position = grid.find_best_location();
        assert_eq!(position, (11, 13));

        assert_eq!(210, grid.count_asteroids(position));
    }

    #[test]
    fn example4() {
        let grid = Map::new(
            ".#..#..###
            ####.###.#
            ....###.#.
            ..###.##.#
            ##.##.#.#.
            ....###..#
            ..#.#..#.#
            #..#.#.###
            .##...##.#
            .....#.#..",
        );

        let position = grid.find_best_location();
        assert_eq!(position, (6, 3));

        assert_eq!(41, grid.count_asteroids(position));
    }

    #[test]
    fn example3() {
        let grid = Map::new(
            "#.#...#.#.
            .###....#.
            .#....#...
            ##.#.#.#.#
            ....#.#.#.
            .##..###.#
            ..#...##..
            ..##....##
            ......#...
            .####.###.",
        );

        let position = grid.find_best_location();
        assert_eq!(position, (1, 2));

        assert_eq!(35, grid.count_asteroids(position));
    }

    #[test]
    fn example2() {
        let grid = Map::new(
            "......#.#.
            #..#.#....
            ..#######.
            .#.#.###..
            .#..#.....
            ..#....#.#
            #..#....#.
            .##.#..###
            ##...#..#.
            .#....####",
        );

        let position = grid.find_best_location();
        assert_eq!(position, (5, 8));

        assert_eq!(33, grid.count_asteroids(position));
    }

    #[test]
    fn example1() {
        let grid = Map::new(
            ".#..#
            .....
            #####
            ....#
            ...##",
        );

        assert_eq!(10, grid.asteroids.len());
        assert_eq!(
            &grid.asteroids,
            &[
                (1, 0),
                (4, 0),
                (0, 2),
                (1, 2),
                (2, 2),
                (3, 2),
                (4, 2),
                (4, 3),
                (3, 4),
                (4, 4)
            ]
        );

        let position = grid.find_best_location();
        assert_eq!(position, (3, 4));

        assert_eq!(4, grid.max_x);
        assert_eq!(4, grid.max_y);
        assert_eq!(8, grid.count_asteroids(position));
    }
}
