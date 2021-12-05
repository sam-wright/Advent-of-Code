use core::panic;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub type Point = (i32, i32);

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

pub fn read_input(filename: &str) -> Vec<Line> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();

        let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        let c = re.captures(&line).unwrap();
        let point1: Point = (
            c.get(1).unwrap().as_str().parse().unwrap(),
            c.get(2).unwrap().as_str().parse().unwrap(),
        );
        let point2: Point = (
            c.get(3).unwrap().as_str().parse().unwrap(),
            c.get(4).unwrap().as_str().parse().unwrap(),
        );

        lines.push(Line {
            start: point1,
            end: point2,
        });
    }
    lines
}

pub fn is_horizontal(line: &Line) -> bool {
    line.start.1 == line.end.1
}
pub fn is_vertical(line: &Line) -> bool {
    line.start.0 == line.end.0
}

pub fn is_diagonal(line: &Line) -> bool {
    (line.start.0 - line.end.0).abs() == (line.start.1 - line.end.1).abs()
}

pub fn draw(lines: &[&Line]) -> HashMap<Point, i32 /* overlaps */> {
    let mut map = HashMap::new();

    for line in lines {
        if is_horizontal(line) {
            let y = line.start.1;
            for x in line.start.0.min(line.end.0)..=line.start.0.max(line.end.0) {
                let p = map.entry((x, y)).or_insert(0);
                *p += 1;
            }
        } else if is_vertical(line) {
            let x = line.start.0;
            for y in line.start.1.min(line.end.1)..=line.start.1.max(line.end.1) {
                let p = map.entry((x, y)).or_insert(0);
                *p += 1;
            }
        }
    }

    map
}

pub fn draw2(lines: &[&Line]) -> HashMap<Point, i32 /* overlaps */> {
    let mut map = HashMap::new();

    for line in lines {
        if is_horizontal(line) {
            let y = line.start.1;
            for x in line.start.0.min(line.end.0)..=line.start.0.max(line.end.0) {
                let p = map.entry((x, y)).or_insert(0);
                *p += 1;
            }
        } else if is_vertical(line) {
            let x = line.start.0;
            for y in line.start.1.min(line.end.1)..=line.start.1.max(line.end.1) {
                let p = map.entry((x, y)).or_insert(0);
                *p += 1;
            }
        } else if is_diagonal(line) {
            let steps = (line.start.0 - line.end.0).abs();
            let dx = (line.end.0 - line.start.0).signum();
            let dy = (line.end.1 - line.start.1).signum();

            for s in 0..=steps {
                let p = map
                    .entry((line.start.0 + s * dx, line.start.1 + s * dy))
                    .or_insert(0);
                *p += 1;
            }
        } else {
            panic!("oops");
        }
    }

    map
}

pub fn count_overlaps(map: &HashMap<Point, i32 /* instances */>) -> i32 {
    map.iter()
        .fold(0, |sum, (_k, v)| if v > &1 { sum + 1 } else { sum })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let lines = read_input("example.txt");

        let filtered_lines: Vec<&Line> = lines
            .iter()
            .filter(|x| is_horizontal(x) || is_vertical(x))
            .collect();

        let map = draw(&filtered_lines);

        let overlaps = count_overlaps(&map);

        assert_eq!(overlaps, 5)
    }

    #[test]
    fn part1() {
        let lines = read_input("input.txt");

        let filtered_lines: Vec<&Line> = lines
            .iter()
            .filter(|x| is_horizontal(x) || is_vertical(x))
            .collect();

        let map = draw(&filtered_lines);

        let overlaps = count_overlaps(&map);

        assert_eq!(overlaps, 8622)
    }

    #[test]
    fn example2() {
        let lines = read_input("example.txt");

        let filtered_lines: Vec<&Line> = lines
            .iter()
            .filter(|x| is_horizontal(x) || is_vertical(x) || is_diagonal(x))
            .collect();

        let map = draw2(&filtered_lines);

        let overlaps = count_overlaps(&map);

        assert_eq!(overlaps, 12)
    }

    #[test]
    fn part2() {
        let lines = read_input("input.txt");

        let filtered_lines: Vec<&Line> = lines
            .iter()
            .filter(|x| is_horizontal(x) || is_vertical(x) || is_diagonal(x))
            .collect();

        let map = draw2(&filtered_lines);

        let overlaps = count_overlaps(&map);

        assert_eq!(overlaps, 22037)
    }
}
