use std::fs::File;
use std::io::Read;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    x: isize,
    y: isize,
    status: Status,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Status {
    Empty,
    Occupied,
    Floor,
}

pub fn read_input(filename: &str) -> Vec<String> {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let collection: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    collection
}

pub fn input_to_positions(input: &Vec<String>) -> Vec<Position> {
    let mut positions = Vec::with_capacity(input.len());

    for (y, line) in input.iter().enumerate() {
        for (x, v) in line.chars().enumerate() {
            positions.push(match v {
                'L' => Position {
                    x: x as isize,
                    y: y as isize,
                    status: Status::Empty,
                },
                '.' => Position {
                    x: x as isize,
                    y: y as isize,
                    status: Status::Floor,
                },
                '#' => Position {
                    x: x as isize,
                    y: y as isize,
                    status: Status::Occupied,
                },
                _ => panic!("invalid status"),
            })
        }
    }
    positions
}

pub fn count_occupied(positions: &Vec<Position>) -> isize {
    positions
        .iter()
        .filter(|&p| p.status == Status::Occupied)
        .count() as isize
}

fn get_position_status(positions: &Vec<Position>, x: isize, y: isize) -> Option<Status> {
    positions
        .iter()
        .find(|&p| p.x == x && p.y == y)
        .map(|p| p.status)
}

// a nice example of abused syntax because I was feeling "clever"... Ew!
fn count_adjacent_occupied(positions: &Vec<Position>, x: isize, y: isize) -> isize {
    [
        if x > 0 {
            get_position_status(&positions, x - 1, y)
        } else {
            None
        },
        get_position_status(&positions, x + 1, y),
        get_position_status(&positions, x, y + 1),
        if y > 0 {
            get_position_status(&positions, x, y - 1)
        } else {
            None
        },
        get_position_status(&positions, x + 1, y + 1),
        if x > 0 {
            get_position_status(&positions, x - 1, y + 1)
        } else {
            None
        },
        if y > 0 {
            get_position_status(&positions, x + 1, y - 1)
        } else {
            None
        },
        if x > 0 && y > 0 {
            get_position_status(&positions, x - 1, y - 1)
        } else {
            None
        },
    ]
    .iter()
    .fold(0, |x, &y| {
        if y == Some(Status::Occupied) {
            x + 1
        } else {
            x
        }
    })
}

fn visible_along_axis(
    positions: &Vec<Position>,
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
) -> isize {
    let mut ds = 0;
    loop {
        ds += 1;
        match get_position_status(positions, x + dx * ds, y + dy * ds) {
            None => return 0,
            Some(Status::Occupied) => return 1,
            Some(Status::Empty) => return 0,
            _ => {}
        }
    }
}

fn count_visible_occupied(positions: &Vec<Position>, x: isize, y: isize) -> isize {
    visible_along_axis(positions, x, y, 1, 0)
        + visible_along_axis(positions, x, y, -1, 0)
        + visible_along_axis(positions, x, y, 0, 1)
        + visible_along_axis(positions, x, y, 0, -1)
        + visible_along_axis(positions, x, y, 1, 1)
        + visible_along_axis(positions, x, y, 1, -1)
        + visible_along_axis(positions, x, y, -1, 1)
        + visible_along_axis(positions, x, y, -1, -1)
}

pub fn evaluate_rules(positions: &Vec<Position>) -> Vec<Position> {
    let mut new_positions = Vec::with_capacity(positions.len());

    for position in positions {
        let mut position = position.clone();

        if position.status == Status::Empty
            && count_adjacent_occupied(&positions, position.x, position.y) == 0
        {
            position.status = Status::Occupied;
        } else if position.status == Status::Occupied
            && count_adjacent_occupied(&positions, position.x, position.y) >= 4
        {
            position.status = Status::Empty;
        }

        new_positions.push(position)
    }
    new_positions
}

pub fn evaluate_new_rules(positions: &Vec<Position>) -> Vec<Position> {
    let mut new_positions = Vec::with_capacity(positions.len());

    for position in positions {
        let mut position = position.clone();

        if position.status == Status::Empty
            && count_visible_occupied(&positions, position.x, position.y) == 0
        {
            position.status = Status::Occupied;
        } else if position.status == Status::Occupied
            && count_visible_occupied(&positions, position.x, position.y) >= 5
        {
            position.status = Status::Empty;
        }

        new_positions.push(position)
    }
    new_positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = read_input("example1.txt");
        let mut positions = input_to_positions(&input);
        let mut count = 0;

        positions = evaluate_rules(&positions);

        loop {
            let new_count = count_occupied(&positions);
            if count == new_count {
                break;
            }
            count = new_count;

            positions = evaluate_rules(&positions);
        }
        assert_eq!(count, 37);
    }

    #[test]
    fn part_1_solution() {
        let input = read_input("input.txt");
        let mut positions = input_to_positions(&input);
        let mut count = 0;

        positions = evaluate_rules(&positions);

        loop {
            let new_count = count_occupied(&positions);
            if count == new_count {
                break;
            }
            count = new_count;

            positions = evaluate_rules(&positions);
        }
        assert_eq!(count, 2338);
    }

    #[test]
    fn part_2_example_a() {
        let input = read_input("example_of_8.txt");
        let positions = input_to_positions(&input);
        let count = count_visible_occupied(&positions, 3, 4);

        assert_eq!(count, 8);
    }

    #[test]
    fn part_2_example_b() {
        let input = read_input("example_of_0_tricky.txt");
        let positions = input_to_positions(&input);
        let count = count_visible_occupied(&positions, 1, 1);

        assert_eq!(count, 0);
    }

    #[test]
    fn part_2_example_c() {
        let input = read_input("example_of_0.txt");
        let positions = input_to_positions(&input);
        let count = count_visible_occupied(&positions, 3, 3);

        assert_eq!(count, 0);
    }

    #[test]
    fn part_2_example() {
        let input = read_input("example1.txt");
        let mut positions = input_to_positions(&input);
        let mut count = 0;

        positions = evaluate_rules(&positions);

        loop {
            let new_count = count_occupied(&positions);
            if count == new_count {
                break;
            }
            count = new_count;
            dbg!(&count);
            positions = evaluate_new_rules(&positions);
        }
        dbg!(&count);

        assert_eq!(count, 26);
    }

    #[test]
    fn part_2_solution() {
        let input = read_input("input.txt");
        let mut positions = input_to_positions(&input);
        let mut count = 0;

        positions = evaluate_rules(&positions);

        loop {
            let new_count = count_occupied(&positions);
            if count == new_count {
                break;
            }
            count = new_count;
            positions = evaluate_new_rules(&positions);
        }

        assert_eq!(count, 2134);
    }
}
