use std::collections::{HashMap, HashSet};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(filename: &str) -> HashMap<(char, char), char> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut rules = HashMap::new();

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let mut data = line.split(" -> ");
        let mut from = data.next().unwrap().chars();
        let to = data.next().unwrap().chars().next().unwrap();

        rules.insert((from.next().unwrap(), from.next().unwrap()), to);
    });

    rules
}

pub fn grow(polymer: String, pair_insertion_rules: &HashMap<(char, char), char>) -> String {
    let mut new_string = String::new();

    let chars: Vec<char> = polymer.chars().collect();
    for idx in 0..polymer.len() - 1 {
        let first = chars.get(idx).unwrap();
        let second = chars.get(idx + 1).unwrap();
        let insertion = pair_insertion_rules.get(&(*first, *second)).unwrap();

        new_string.push(*first);
        new_string.push(*insertion);
        // new_string.push(*second);
    }

    new_string.push(polymer.chars().into_iter().rev().next().unwrap());

    new_string
}

pub fn get_score(polymer: &str) -> i32 {
    let letters: HashSet<char> = polymer.chars().collect();
    let mut max = i32::MIN;
    let mut min = i32::MAX;

    for letter in letters {
        let count = polymer.matches(letter).count() as i32;
        if count > max {
            max = count;
        }
        if count < min {
            min = count;
        }
    }
    max - min
}

pub fn polymer2hashy<T>(polymer: T) -> HashMap<(char, char), usize>
where
    T: ToString,
{
    let mut hashy = HashMap::new();

    let chars: Vec<char> = polymer.to_string().chars().collect();
    for idx in 0..polymer.to_string().len() - 1 {
        let first = chars.get(idx).unwrap();
        let second = chars.get(idx + 1).unwrap();

        let e = hashy.entry((*first, *second)).or_insert(0);
        *e += 1;
    }

    hashy
}

pub fn grow2(
    polymer: &HashMap<(char, char), usize>,
    pair_insertion_rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    let mut new_polymer = HashMap::new();

    for (k, v) in polymer {
        let first = k.0;
        let second = k.1;
        let insertion = *pair_insertion_rules.get(&(first, second)).unwrap();

        let e = new_polymer.entry((first, insertion)).or_insert(0);
        *e += v;

        let e = new_polymer.entry((insertion, second)).or_insert(0);
        *e += v;
    }

    new_polymer
}

pub fn get_frequencies(polymer: &HashMap<(char, char), usize>) -> HashMap<char, usize> {
    let mut firsts = HashMap::new();
    let mut seconds = HashMap::new();

    for (_i, (k, v)) in polymer.iter().enumerate() {
        let e = firsts.entry(k.0).or_insert(0);
        *e += v;

        let e = seconds.entry(k.1).or_insert(0);
        *e += v;
    }

    let mut freqs = firsts;
    for (k, v) in seconds {
        let e = freqs.entry(k).or_insert(0);
        if *e < v {
            *e = v;
        }
    }

    freqs
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example1() {
        let pair_insertion_rules = read_input("example.txt");
        let polymer_template = "NNCB".to_string();

        // Step 1
        let polymer = grow(polymer_template.clone(), &pair_insertion_rules);
        assert_eq!(&polymer, "NCNBCHB");

        //Step 2
        let polymer = grow(polymer, &pair_insertion_rules);
        assert_eq!(&polymer, "NBCCNBBBCBHCB");

        // Step 3
        let polymer = grow(polymer, &pair_insertion_rules);
        assert_eq!(&polymer, "NBBBCNCCNBBNBNBBCHBHHBCHB");

        // Step 4
        let polymer = grow(polymer, &pair_insertion_rules);
        assert_eq!(
            &polymer,
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );

        // Step 5
        let polymer = grow(polymer, &pair_insertion_rules);
        assert_eq!(97, polymer.len());

        let mut poly_10 = grow(polymer_template.clone(), &pair_insertion_rules);
        for _ in 0..9 {
            poly_10 = grow(poly_10, &pair_insertion_rules);
        }
        assert_eq!(3073, poly_10.len());

        let score = get_score(&poly_10);
        assert_eq!(score, 1588);
    }

    #[test]
    fn part1() {
        let pair_insertion_rules = read_input("input.txt");
        let polymer_template = "KHSSCSKKCPFKPPBBOKVF".to_string();

        let mut poly_10 = grow(polymer_template, &pair_insertion_rules);
        for _ in 0..9 {
            poly_10 = grow(poly_10, &pair_insertion_rules);
        }

        let score = get_score(&poly_10);
        assert_eq!(score, 5656);
    }

    #[test]
    fn example2() {
        let pair_insertion_rules = read_input("example.txt");
        let polymer_template = "NNCB";

        let mut polymer = polymer2hashy(&polymer_template);

        // step 1
        polymer = grow2(&polymer, &pair_insertion_rules);
        assert_eq!(&polymer, &polymer2hashy("NCNBCHB"));

        // step 2
        polymer = grow2(&polymer, &pair_insertion_rules);
        assert_eq!(&polymer, &polymer2hashy("NBCCNBBBCBHCB"));

        // step 3
        polymer = grow2(&polymer, &pair_insertion_rules);
        assert_eq!(&polymer, &polymer2hashy("NBBBCNCCNBBNBNBBCHBHHBCHB"));

        let mut poly_10 = polymer2hashy(&polymer_template);
        for _ in 0..10 {
            poly_10 = grow2(&poly_10, &pair_insertion_rules);
        }
        let frequencies = get_frequencies(&poly_10);

        let score = frequencies
            .iter()
            .fold(usize::MIN, |max, (_k, &v)| if v > max { v } else { max })
            - frequencies
                .iter()
                .fold(usize::MAX, |min, (_k, &v)| if v < min { v } else { min });

        assert_eq!(score, 1588);

        let mut poly_40 = polymer2hashy(&polymer_template);
        for _ in 0..40 {
            poly_40 = grow2(&poly_40, &pair_insertion_rules);
        }

        let frequencies = get_frequencies(&poly_40);

        let score = frequencies
            .iter()
            .fold(usize::MIN, |max, (_k, &v)| if v > max { v } else { max })
            - frequencies
                .iter()
                .fold(usize::MAX, |min, (_k, &v)| if v < min { v } else { min });

        assert_eq!(score, 2188189693529)
    }

    #[test]
    fn part2() {
        let pair_insertion_rules = read_input("input.txt");
        let polymer_template = "KHSSCSKKCPFKPPBBOKVF";

        let mut polymer = polymer2hashy(&polymer_template);
        for _ in 0..40 {
            polymer = grow2(&polymer, &pair_insertion_rules);
        }

        let frequencies = get_frequencies(&polymer);

        let score = frequencies
            .iter()
            .fold(usize::MIN, |max, (_k, &v)| if v > max { v } else { max })
            - frequencies
                .iter()
                .fold(usize::MAX, |min, (_k, &v)| if v < min { v } else { min });

        assert_eq!(score, 12271437788530)
    }
}
