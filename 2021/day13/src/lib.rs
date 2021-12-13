use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(filename: &str) -> HashSet<(i32, i32)> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let mut coords = x.split(',');
            (
                coords.next().unwrap().clone().parse().unwrap(),
                coords.next().unwrap().clone().parse().unwrap(),
            )
        })
        .collect()
}

pub fn fold_along_x(x_mirror: i32, dots: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new_dots = HashSet::new();

    for dot in dots {
        if dot.0.abs() > x_mirror.abs() {
            let dx = x_mirror - dot.0;
            new_dots.insert((dot.0 + 2 * dx, dot.1));
        } else {
            new_dots.insert(*dot);
        }
    }

    new_dots
}

pub fn fold_along_y(y_mirror: i32, dots: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new_dots = HashSet::new();

    for dot in dots {
        if dot.1.abs() > y_mirror.abs() {
            let dy = y_mirror - dot.1;
            new_dots.insert((dot.0, dot.1 + 2 * dy));
        } else {
            new_dots.insert(*dot);
        }
    }

    new_dots
}

pub fn print_dots(dots: &HashSet<(i32, i32)>) {
    for y in -10..10 {
        for x in -60..60 {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example1() {
        let dots = read_input("example.txt");

        let dots = fold_along_y(7, &dots);
        print_dots(&dots);
        assert_eq!(dots.len(), 17);

        let dots = fold_along_x(5, &dots);
        print_dots(&dots);
    }
    #[test]
    fn part1() {
        let dots = read_input("input.txt");

        let dots = fold_along_x(655, &dots);
        assert_eq!(dots.len(), 706);
    }

    #[test]
    fn part2() {
        let dots = read_input("input.txt");

        let dots = fold_along_x(655, &dots);
        let dots = fold_along_y(447, &dots);
        let dots = fold_along_x(327, &dots);
        let dots = fold_along_y(223, &dots);
        let dots = fold_along_x(163, &dots);
        let dots = fold_along_y(111, &dots);
        let dots = fold_along_x(81, &dots);
        let dots = fold_along_y(55, &dots);
        let dots = fold_along_x(40, &dots);
        let dots = fold_along_y(27, &dots);
        let dots = fold_along_y(13, &dots);
        let dots = fold_along_y(6, &dots);

        print_dots(&dots);
        // prints out LRFJBJEH
    }
}
