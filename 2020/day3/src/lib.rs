use std::fs::File;
use std::io::Read;

pub fn parse_input(filename: &str) -> Vec<String> {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let collection: Vec<String> = contents.split('\n').map(|x| x.to_string()).collect();
    collection
}

pub fn count_hits(rows: &Vec<String>, dx: i32, dy: i32) -> usize {
    let mut x_pos = dx as usize;
    let mut y_pos = dy as usize;
    let mut hits = 0;

    while y_pos < rows.len() {
        let row = &rows[y_pos];
        if x_pos > row.len() - 1 {
            x_pos -= row.len()
        }

        if row.chars().nth(x_pos).unwrap() == '#' {
            hits += 1;
        // println!("hits: {}", hits);
        } else {
        }
        y_pos += dy as usize;
        x_pos += dx as usize;
    }
    hits
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn part_1_example() {
        let rows = parse_input("example1.txt");

        for r in &rows {
            println!("{}", r);
        }

        assert_eq!(count_hits(&rows, 3, 1), 7);
    }

    #[test]
    fn part_1_solution() {
        let rows = parse_input("input.txt");

        assert_eq!(count_hits(&rows, 3, 1), 244);
    }

    #[test]
    fn part_2_example() {
        let rows = parse_input("example1.txt");

        assert_eq!(count_hits(&rows, 1, 1), 2);
        assert_eq!(count_hits(&rows, 3, 1), 7);
        assert_eq!(count_hits(&rows, 5, 1), 3);
        assert_eq!(count_hits(&rows, 7, 1), 4);
        assert_eq!(count_hits(&rows, 1, 2), 2);
    }

    #[test]
    fn part_2_solution() {
        let rows = parse_input("input.txt");

        let ans = count_hits(&rows, 1, 1)
            * count_hits(&rows, 3, 1)
            * count_hits(&rows, 5, 1)
            * count_hits(&rows, 7, 1)
            * count_hits(&rows, 1, 2);

        assert_eq!(ans, 9406609920);
    }
}
