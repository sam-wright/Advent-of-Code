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
        if !is_valid(&cypher, input[i]) {
            return input[i];
        }

        cypher.pop_back();
        cypher.push_front(input[i]);
    }

    // failure
    -1
}

pub fn find_weakness_set(input: &Vec<i64>, error: i64) -> Vec<i64> {
    for i in 0..input.len() - 1 {
        let mut k = 0;
        let mut value = 0;
        loop {
            value += input[i + k];

            if value == error {
                let ans: Vec<i64> = input[i..=i + k].iter().map(|x| x.clone()).collect();
                return ans;
            }

            if value > error {
                break;
            }
            k += 1;
        }
    }

    //failure
    return Vec::new();
}

pub fn find_weakness_set_improved(input: &Vec<i64>, error: i64) -> VecDeque<i64> {
    let mut weakness_set = VecDeque::new();
    let mut input: VecDeque<i64> = input.iter().map(|x|{x.clone()}).collect();
    loop {
        let value: i64 = weakness_set.iter().sum();
        if value > error {
            weakness_set.pop_back();
        } else if value == error {
            break;
        } else {
            weakness_set.push_front(input.pop_front().unwrap());
        }
    }

    weakness_set
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

        let pair = find_weakness_set(&input, error);
        let ans = pair.iter().max().unwrap() + pair.iter().min().unwrap();

        assert_eq!(ans, 62);
    }

    #[test]
    fn part_2_solution() {
        let input = read_input("input.txt");
        let error = find_error(&input, 25);

        let pair = find_weakness_set(&input, error);
        let ans = pair.iter().max().unwrap() + pair.iter().min().unwrap();

        assert_eq!(ans, 177989832);
    }

    #[test]
    fn part_2_solution_improved() {
        let input = read_input("input.txt");
        let error = find_error(&input, 25);

        let pair = find_weakness_set_improved(&input, error);
        let ans = pair.iter().max().unwrap() + pair.iter().min().unwrap();

        assert_eq!(ans, 177989832);
    }
}
