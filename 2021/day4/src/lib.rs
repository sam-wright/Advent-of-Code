use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// type Board = [[i32; 5]; 5];

// Row-Oriented matrix
// type Board = [i32; 25];

// Simple
type Row = Vec<(bool, i32)>;
type Board = Vec<Row>;

type Moves = Vec<i32>;

pub fn check_rows(board: &Board) -> bool {
    'row: for row in board {
        for entry in row {
            if entry.0 == false {
                continue 'row;
            }
        }
        println!("Bingo (row)");
        // dbg!(board);
        return true;
    }

    false
}

pub fn check_cols(board: &Board) -> bool {
    'col: for idx in 0..5 {
        for row in board {
            if row[idx].0 == false {
                continue 'col;
            }
        }
        println!("Bingo (col)");

        return true;
    }

    false
}

pub fn read_input(filename: &str) -> (Moves, Vec<Board>) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    // strip off the trailing newline
    let line = line.replace('\n', "");

    let moves: Moves = line.split(',').map(|s| s.parse().unwrap()).collect();

    let mut boards = Vec::new();
    // let mut idx = 0;
    let mut tmp_board: Board = Vec::new();

    // consume that emptyline so that we dont create an empty board
    reader.consume(1);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() > 1 {
            let row: Row = line
                .split_ascii_whitespace()
                .map(|s| (false, s.parse().unwrap()))
                .collect();
            tmp_board.push(row);
        } else {
            boards.push(tmp_board.clone());
            tmp_board.clear();
        }
    }
    boards.push(tmp_board.clone());

    (moves, boards)
}

pub fn mark_boards(number: i32, boards: &Vec<Board>) -> Vec<Board> {
    let mut return_boards = Vec::new();

    for board in boards {
        let mut new_board = board.clone();
        for row in &mut new_board {
            for mut entry in row {
                if entry.1 == number {
                    entry.0 = true;
                }
            }
        }
        return_boards.push(new_board);
    }

    return_boards
}

pub fn calculate_score(board: &Board, number: i32) -> i32 {
    // let mut score = 0;

    let score = board.iter().flatten().fold(
        0,
        |score, (visited, val)| {
            if !visited {
                val + score
            } else {
                score
            }
        },
    );

    score * number
}
pub fn print_board(board: &Board) {
    for b in board {
        println!("{:?}", b);
    }
}

#[cfg(test)]
mod tests {
    use std::process::exit;

    use super::*;
    #[test]
    fn example1() {
        let (moves, mut boards) = read_input("example.txt");

        for number in moves {
            println!("Playing {}...", number);
            boards = mark_boards(number, &boards);

            for board in &boards {
                if check_cols(&board) || check_rows(&board) {
                    println!("Number {} did it!", number);
                    assert_eq!(4512, calculate_score(&board, number));
                    exit(0);
                }
            }
        }
    }

    #[test]
    fn part1() {
        let (moves, mut boards) = read_input("input.txt");

        for number in moves {
            println!("Playing {}...", number);
            boards = mark_boards(number, &boards);

            for board in &boards {
                if check_cols(&board) || check_rows(&board) {
                    println!("Number {} did it!", number);
                    assert_eq!(67716, calculate_score(&board, number));
                    exit(0);
                }
            }
        }
    }

    #[test]
    fn example2() {
        let (moves, mut boards) = read_input("example.txt");

        for number in moves {
            println!("Playing {}...", number);
            let updated_boards = mark_boards(number, &boards);

            let mut new_boards = Vec::new();
            for board in &updated_boards {
                if check_cols(&board) || check_rows(&board) {
                    print_board(&board);
                    continue;
                }
                new_boards.push(board.clone());
            }
            if updated_boards.len() == 1 {
                assert_eq!(
                    1924,
                    calculate_score(&updated_boards.first().unwrap(), number)
                );
            }
            boards = new_boards;
        }
    }

    #[test]
    fn part2() {
        let (moves, mut boards) = read_input("input.txt");

        for number in moves {
            println!("Playing {}...", number);
            let updated_boards = mark_boards(number, &boards);

            let mut new_boards = Vec::new();
            for board in &updated_boards {
                if check_cols(&board) || check_rows(&board) {
                    print_board(&board);
                    continue;
                }
                new_boards.push(board.clone());
            }
            if updated_boards.len() == 1 {
                assert_eq!(
                    1830,
                    calculate_score(&updated_boards.first().unwrap(), number)
                );
            }
            boards = new_boards;
        }
    }
}
