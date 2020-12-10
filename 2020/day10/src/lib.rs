#![recursion_limit = "8192"]
use std::fs::File;
use std::io::Read;

pub fn read_adapters(filename: &str) -> Vec<i32> {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let collection: Vec<i32> = contents.split("\n").map(|x| x.parse().unwrap()).collect();
    collection
}

pub fn order_adapters(adapters: &Vec<i32>) -> Vec<i32> {
    let mut ordered_adapters = vec![0];
    ordered_adapters.append(&mut adapters.clone());
    ordered_adapters.sort();

    ordered_adapters.push(ordered_adapters.iter().max().unwrap() + 3);

    ordered_adapters
}

pub fn get_n_jolt_differences(joltages: &Vec<i32>, diff_size: i32) -> i32 {
    let mut diff = 0;

    for i in 0..joltages.len() - 1 {
        if joltages[i + 1] - joltages[i] == diff_size {
            diff += 1;
        }
    }
    diff
}

// This algorithmically works, but might take the age of the universe to complete...
fn explore(joltages: &Vec<i32>) -> usize {
    let mut solutions = 0;
    let mut exploration: Vec<Vec<i32>> = vec![vec![joltages[0]]];

    let max_joltage = joltages.iter().max().unwrap();
    while exploration.len() > 0 {
        let instance = exploration.pop().unwrap();
        let instance_max = instance.iter().max().unwrap();
        let idx = joltages.iter().position(|x| x == instance_max).unwrap();

        for step in 1..=3 {
            if idx + step >= joltages.len() {
                continue;
            }
            if joltages[idx + step] - joltages[idx] <= 3 {
                if joltages[idx + step] == *max_joltage {
                    solutions += 1;
                    continue;
                }
                let mut new_instance = instance.clone();
                new_instance.push(joltages[idx + step]);
                exploration.push(new_instance)
            }
        }
    }
    solutions
}

fn exploreDP(joltages: &Vec<i32>) -> usize {
    let  joltages = joltages.clone();
    let len = joltages.len();

    let mut pow2 = 0;
    let mut pow7 = 0;

    for i in 1..len - 1 {
        if i >= 3 && joltages[i + 1] - joltages[i - 3] == 4 {
            pow7 += 1;
            pow2 -= 2;
        } else if joltages[i + 1] - joltages[i - 1] == 2 {
            pow2 += 1;
        }
    }

    2_usize.pow(pow2) * 7_usize.pow(pow7)
}

pub fn count_arrangements(joltages: &Vec<i32>) -> usize {
    // explore(joltages) // caveat emptor!
    exploreDP(joltages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let adapters = read_adapters("example1.txt");
        let ordered_adapters = order_adapters(&adapters);

        let n1 = get_n_jolt_differences(&ordered_adapters, 1);
        assert_eq!(n1, 22);

        let n3 = get_n_jolt_differences(&ordered_adapters, 3);
        assert_eq!(n3, 10);
    }

    #[test]
    fn part_1_solution() {
        let adapters = read_adapters("input.txt");
        let ordered_adapters = order_adapters(&adapters);

        let n1 = get_n_jolt_differences(&ordered_adapters, 1);
        assert_eq!(n1, 65);

        let n3 = get_n_jolt_differences(&ordered_adapters, 3);
        assert_eq!(n3, 26);

        println!("Part1 solution: {}", n1 * n3);
    }

    #[test]
    fn part_2_example() {
        let adapters = read_adapters("example1.txt");
        let ordered_adapters = order_adapters(&adapters);

        let arrangements = count_arrangements(&ordered_adapters);

        assert_eq!(arrangements, 19208);
    }

    #[test]
    fn part_2_solution() {
        let adapters = read_adapters("input.txt");
        let ordered_adapters = order_adapters(&adapters);

        let arrangements = count_arrangements(&ordered_adapters);
        println!("Part2 solution: {}", arrangements);

        assert_eq!(arrangements, 5289227976704);
    }
}
