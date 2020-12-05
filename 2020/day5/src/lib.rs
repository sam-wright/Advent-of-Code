fn get_search_space(size: i32) -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..size {
        v.push(i);
    }
    v
}

fn left_half(space: &Vec<i32>) -> Vec<i32> {
    let len = space.len();

    space[0..(len / 2)].to_vec()
}

fn right_half(space: &Vec<i32>) -> Vec<i32> {
    let len = space.len();

    space[(len / 2)..len].to_vec()
}

pub fn get_row(input: &str) -> i32 {
    let mut ss = get_search_space(128);

    for i in 0..7 {
        ss = match input.chars().nth(i).unwrap() {
            'F' => left_half(&ss),
            'B' => right_half(&ss),
            _ => panic!("Bad input"),
        };
    }

    ss[0]
}

pub fn get_column(input: &str) -> i32 {
    let mut ss = get_search_space(8);
    let offset = 7;

    for i in 0..3 {
        ss = match input.chars().nth(offset + i).unwrap() {
            'L' => left_half(&ss),
            'R' => right_half(&ss),
            _ => panic!("Bad input"),
        };
    }

    ss[0]
}

pub fn compute_seat_id(input: &str) -> i32 {
    get_row(input) * 8 + get_column(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    pub fn parse_input() -> Vec<String> {
        let mut contents = String::new();
        let mut file = File::open("input.txt").unwrap();
        file.read_to_string(&mut contents).unwrap();

        let collection: Vec<String> = contents.split('\n').map(|x| x.to_string()).collect();
        collection
    }

    #[test]
    fn get_ss_test() {
        assert_eq!(get_search_space(1), [0]);
        assert_eq!(get_search_space(2), [0, 1]);
        assert_eq!(get_search_space(8), [0, 1, 2, 3, 4, 5, 6, 7]);
    }
    #[test]
    fn left_half_test() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(left_half(&v), [1, 2]);

        let w = vec![1, 2];
        assert_eq!(left_half(&w), [1]);
    }

    #[test]
    fn right_half_test() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(right_half(&v), [3, 4]);

        let w = vec![1, 2];
        assert_eq!(right_half(&w), [2]);
    }

    #[test]
    fn part_1_example() {
        let ex1 = "BFFFBBFRRR";
        let ex2 = "FFFBBBFRRR";
        let ex3 = "BBFFBBFRLL";

        assert_eq!(get_column(ex1), 7);
        assert_eq!(get_row(ex1), 70);

        assert_eq!(get_column(ex2), 7);
        assert_eq!(get_row(ex2), 14);

        assert_eq!(get_column(ex3), 4);
        assert_eq!(get_row(ex3), 102);

        let ex4 = "FBFBBFFRLR";
        assert_eq!(compute_seat_id(ex4), 357);
    }

    #[test]
    fn part_1_solution() {
        let input = parse_input();

        let max_id = input.iter().fold(0, |max, line| {
            let id = compute_seat_id(line);
            if id > max {
                id
            } else {
                max
            }
        });

        println!("Max ID: {}", max_id);
        assert_eq!(max_id, 970);
    }

    #[test]
    fn part_2_solution() {
        let input = parse_input();

        let max_id = 971;
        let mut ss = get_search_space(max_id);

        for line in input {
            let id = compute_seat_id(&line);
            ss[id as usize] = 0;
        }

        let my_id = *ss.iter().max().unwrap();

        println!("My ID: {}", my_id);
        assert_eq!(my_id, 587);
    }
}
