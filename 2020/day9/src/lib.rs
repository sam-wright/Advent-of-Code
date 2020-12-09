use std::collections::vec_deque::VecDeque;
use std::fs::File;
use std::io::Read;

pub fn read_input(filename: &str) -> Vec<i64> {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let collection: Vec<i64> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
    collection
}

fn is_valid(cypher: &VecDeque<i64>, entry: i64) -> bool {
    for j in 0..cypher.len() {
        for k in 0..cypher.len() {
            if j == k {
                continue;
            }
            if cypher[j] + cypher[k] == entry {
                return true;
            }
        }
    }

    false
}

pub fn find_error(input: &Vec<i64>, preamble_size: usize) -> i64 {
    let mut cypher: VecDeque<i64> = VecDeque::with_capacity(preamble_size);

    // preload the cypher
    for i in 0..preamble_size {
        cypher.push_front(input[i]);
    }

    // process data
    for i in preamble_size as usize..input.len() {
        match is_valid(&cypher, input[i]) {
            true => {}
            false => return input[i],
        }
        cypher.pop_back();
        cypher.push_front(input[i]);
    }

    -1
}

pub fn find_pair(input: &Vec<i64>, error: i64) -> Vec<i64> {
    for i in 0..input.len() - 1 {
        let mut k = 0;
        let mut value = 0;
        loop {
            value += input[i + k];

            if value == error {
                println!("success!");
                let mut ans = Vec::new();
                for j in i..=i + k {
                    ans.push(input[j]);
                }
            }

            if value > error {
                break;
            }
            k += 1;
        }
    }
    return Vec::new();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_example() {
        let input = read_input("example1.txt");
        let error = find_error(&input, 5);

        assert_eq!(error, 127);
    }
    #[test]
    fn part_1_solution() {
        let input = read_input("input.txt");
        let error = find_error(&input, 25);

        assert_eq!(error, 1309761972);
    }

    #[test]
    fn part_2_example() {
        let input = read_input("example1.txt");
        let error = find_error(&input, 5);

        let pair = find_pair(&input, error);
        let ans = pair.iter().max().unwrap() + pair.iter().min().unwrap();

        assert_eq!(ans, 62);
    }

    #[test]
    fn part_2_solution() {
        let input = read_input("input.txt");
        let error = find_error(&input, 25);

        let pair = find_pair(&input, error);
        let ans = pair.iter().max().unwrap() + pair.iter().min().unwrap();

        assert_eq!(ans, 177989832);
    }
}
