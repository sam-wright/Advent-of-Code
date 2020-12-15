use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn read_input(filename: &str) -> Vec<usize> {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let collection: Vec<usize> = contents.split(",").map(|x| x.parse().unwrap()).collect();
    collection
}

pub fn run(input_1: &Vec<usize>, num_turns: usize) -> usize {
    // This will be a map of <number_spoken, turn_spoken>
    let mut memory = HashMap::new();
    let mut prev = 0;

    // Store up the seed turns
    for (i, num) in input_1.iter().enumerate() {
        memory.insert(*num, i);
        prev = *num;
    }

    memory.remove(input_1.last().unwrap()); // we have't committed this to memory (yet)

    // Now run wild!
    for i in input_1.len() - 1..=num_turns - 2 {
        let val = if memory.contains_key(&prev) {
            // Number has been said before
            let ret = i - memory[&prev];
            memory.insert(prev, i);

            ret
        } else {
            // New number
            memory.insert(prev, i);

            0
        };
        prev = val;
    }

    prev
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_1_example() {
        let input_1 = read_input("example1.txt");
        let input_2 = vec![1, 3, 2];
        let input_3 = vec![2, 1, 3];
        let input_4 = vec![1, 2, 3];
        let input_5 = vec![2, 3, 1];
        let input_6 = vec![3, 2, 1];
        let input_7 = vec![3, 1, 2];

        assert_eq!(run(&input_1, 4), 0);
        assert_eq!(run(&input_1, 5), 3);
        assert_eq!(run(&input_1, 6), 3);
        assert_eq!(run(&input_1, 7), 1);
        assert_eq!(run(&input_1, 8), 0);
        assert_eq!(run(&input_1, 9), 4);
        assert_eq!(run(&input_1, 10), 0);

        assert_eq!(run(&input_1, 2020), 436);
        assert_eq!(run(&input_2, 2020), 1);
        assert_eq!(run(&input_3, 2020), 10);
        assert_eq!(run(&input_4, 2020), 27);
        assert_eq!(run(&input_5, 2020), 78);
        assert_eq!(run(&input_6, 2020), 438);
        assert_eq!(run(&input_7, 2020), 1836);
    }

    #[test]
    fn part_1_solution() {
        let input_1 = read_input("input.txt");

        assert_eq!(run(&input_1, 2020), 517);
    }

    #[test]
    fn part_2_solution() {
        let input_1 = read_input("input.txt");
        assert_eq!(run(&input_1, 30000000), 517);
    }
}
