use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;

pub fn parse_input() -> Vec<i32> {
    let mut contents = String::new();
    let mut file = File::open("input.txt").unwrap();
    file.read_to_string(&mut contents).unwrap();

    let collection: Vec<&str> = contents.split('\n').collect();
    let data = || {
        let mut data = Vec::new();
        for c in collection {
            data.push(c.parse().unwrap());
        }
        data
    };
    data()
}

pub fn find_pairs(data: Vec<i32>) -> (i32, i32) {
    let ans = 2020;
    let map: BTreeSet<i32> = data.iter().cloned().collect();

    for m in &map {
        for n in &map {
            if m == n {
                continue;
            }

            if m + n == ans {
                return (*m, *n);
            }
        }
    }

    panic!("unable to solve!")
}

pub fn find_triple(data: Vec<i32>) -> (i32, i32, i32) {
    let ans = 2020;
    let map: BTreeSet<i32> = data.iter().cloned().collect();

    for m in &map {
        for n in &map {
            for p in &map {
                if m == n || m == p || n == p {
                    continue;
                }

                if m + n + p == ans {
                    return (*m, *n, *p);
                }
            }
        }
    }

    panic!("unable to solve!")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_example() {
        let data = vec![1721, 979, 366, 299, 675, 1456];
        let pair = find_pairs(data);

        println!(
            "Multiplying the entries together produces: {} * {} = {}",
            pair.0,
            pair.1,
            pair.0 * pair.1
        );
    }

    #[test]
    fn part_1_solution() {
        let data = parse_input();
        let pair = find_pairs(data);

        println!(
            "Multiplying the entries together produces: {} * {} = {}",
            pair.0,
            pair.1,
            pair.0 * pair.1
        );
    }

    #[test]
    fn part_2_example() {
        let data = vec![1721, 979, 366, 299, 675, 1456];
        let triple = find_triple(data);

        println!(
            "Multiplying the entries together produces: {} * {} * {} = {}",
            triple.0,
            triple.1,
            triple.2,
            triple.0 * triple.1 * triple.2
        );
    }

    #[test]
    fn part_2_solution() {
        let data = parse_input();
        let triple = find_triple(data);

        println!(
            "Multiplying the entries together produces: {} * {} * {} = {}",
            triple.0,
            triple.1,
            triple.2,
            triple.0 * triple.1 * triple.2
        );
    }
}
