use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub enum Op {
    AddX(i32),
    NoOp,
}
pub fn read_input(filename: &str) -> Vec<Op> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            if line.contains("noop") {
                Op::NoOp
            } else if line.contains("addx") {
                let v: Vec<&str> = line.split(' ').collect();
                Op::AddX(v[1].parse().unwrap())
            } else {
                panic!("arst");
            }
        })
        .collect()
}

pub fn execute(ops: &[Op], cycles: i32) -> i32 {
    let mut ops: VecDeque<&Op> = ops.iter().collect();
    let mut x = 1;
    let mut op = ops.pop_front().unwrap_or(&Op::NoOp);
    let mut add_delay = false;

    for cycle in 0..cycles {
        // handle op timing
        match op {
            Op::AddX(_) => {
                if !add_delay {
                    add_delay = true;
                    continue;
                }
            }
            Op::NoOp => {
                op = ops.pop_front().unwrap_or(&Op::NoOp);
                continue;
            }
        }

        if cycle == cycles - 1 {
            return x;
        }

        // handle op effect
        if let Op::AddX(v) = op {
            x += v;
            op = ops.pop_front().unwrap_or(&Op::NoOp);
            add_delay = false;
        }
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_b() {
        let ops = read_input("example2.txt");
        assert_eq!(execute(&ops, 20), 21);
        assert_eq!(execute(&ops, 60), 19);
        assert_eq!(execute(&ops, 100), 18);
        assert_eq!(execute(&ops, 140), 21);
        assert_eq!(execute(&ops, 180), 16);
        assert_eq!(execute(&ops, 220), 18);
    }

    #[test]
    fn part1() {
        let ops = read_input("input.txt");

        let v = execute(&ops, 20) * 20
            + execute(&ops, 60) * 60
            + execute(&ops, 100) * 100
            + execute(&ops, 140) * 140
            + execute(&ops, 180) * 180
            + execute(&ops, 220) * 220;

        assert_eq!(v, 13820);
    }

    #[test]
    fn example2() {
        let ops = read_input("example2.txt");
        let mut row_offset = 0;

        for _ in 0..6 {
            for c in 0..40 {
                let sprite_position = execute(&ops, c + row_offset + 1);
                if c == sprite_position - 1 || c == sprite_position || c == sprite_position + 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            row_offset += 40;
            println!();
        }
    }

    #[test]
    fn part2() {
        let ops = read_input("input.txt");
        let mut row_offset = 0;

        for _ in 0..6 {
            for c in 0..40 {
                let sprite_position = execute(&ops, c + row_offset + 1);
                if c == sprite_position - 1 || c == sprite_position || c == sprite_position + 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            row_offset += 40;
            println!();
        }
    }
}
