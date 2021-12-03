use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|x| x.unwrap()).collect()
}

pub fn most_common(input: &[String]) -> Vec<bool> {
    let mut mask: Vec<i32> = vec![0; input[0].len()];

    for val in input {
        for (idx, bit) in val.chars().enumerate() {
            match bit {
                '0' => {}
                '1' => {
                    mask[idx] += 1;
                }
                _ => panic!("bad input"),
            }
        }
    }

    mask.iter()
        .map(|x| *x as f32 >= (input.len() as f32 / 2.0))
        .collect()
}

pub fn mask_to_int(mask: &[bool]) -> i32 {
    mask.iter().rev().enumerate().fold(0, |val, (idx, x)| {
        if x == &true {
            val + 2i32.pow(idx as u32)
        } else {
            val
        }
    })
}

pub fn invert_mask(mask: &[bool]) -> Vec<bool> {
    mask.iter().map(|x| !x).collect()
}

pub fn filter_mask_common(input: &Vec<String>) -> Vec<bool> {
    let mut input = input.clone();

    // for each column
    for idx in 0..input.len() {
        if input.len() == 1 {
            break;
        }
        let common = most_common(&input);
        let key = if common[idx] == true { '1' } else { '0' };

        let mut new_input = Vec::new();
        for i in input {
            if i.chars().nth(idx).unwrap() == key {
                new_input.push(i);
            }
        }
        input = new_input;
    }
    input
        .first()
        .unwrap()
        .chars()
        .map(|x| match x {
            '0' => false,
            '1' => true,
            _ => panic!("bad input"),
        })
        .collect()
}

pub fn filter_mask_uncommon(input: &Vec<String>) -> Vec<bool> {
    let mut input = input.clone();

    // for each column
    for idx in 0..input.len() {
        if input.len() == 1 {
            break;
        }
        let common = most_common(&input);
        let key = if common[idx] == true { '0' } else { '1' }; // simple hack to flip common->uncommon

        let mut new_input = Vec::new();
        for i in input {
            if i.chars().nth(idx).unwrap() == key {
                new_input.push(i);
            }
        }
        input = new_input;
    }
    input
        .first()
        .unwrap()
        .chars()
        .map(|x| match x {
            '0' => false,
            '1' => true,
            _ => panic!("bad input"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let input = read_input("example.txt");

        let gamma_mask = most_common(&input);
        let epsilon_mask = invert_mask(&gamma_mask);
        assert_eq!(gamma_mask, [true, false, true, true, false]);
        assert_eq!(epsilon_mask, [false, true, false, false, true]);

        let gamma = mask_to_int(&gamma_mask);
        let epsilon = mask_to_int(&epsilon_mask);
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
    }

    #[test]
    fn part1() {
        let input = read_input("input.txt");

        let gamma_mask = most_common(&input);
        let epsilon_mask = invert_mask(&gamma_mask);

        let gamma = mask_to_int(&gamma_mask);
        let epsilon = mask_to_int(&epsilon_mask);
        assert_eq!(gamma, 805);
        assert_eq!(epsilon, 3290);

        println!("{}", gamma * epsilon);
    }

    #[test]
    fn example2() {
        let input = read_input("example.txt");

        let oxygen_mask = filter_mask_common(&input);
        let oxygen = mask_to_int(&oxygen_mask);
        assert_eq!(oxygen_mask, [true, false, true, true, true]);
        assert_eq!(oxygen, 23);

        let co2_mask = filter_mask_uncommon(&input);
        let co2 = mask_to_int(&co2_mask);
        assert_eq!(co2_mask, [false, true, false, true, false]);
        assert_eq!(co2, 10);
    }

    #[test]
    fn part2() {
        let input = read_input("input.txt");

        let oxygen_mask = filter_mask_common(&input);
        let oxygen = mask_to_int(&oxygen_mask);
        assert_eq!(oxygen, 841);

        let co2_mask = filter_mask_uncommon(&input);
        let co2 = mask_to_int(&co2_mask);
        assert_eq!(co2, 3384);

        println!("{}", oxygen * co2);
    }
}
