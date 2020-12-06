use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

pub fn read_groups(filename: &str) -> Vec<String> {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let collection: Vec<String> = contents.split("\n\n").map(|x| x.to_string()).collect();
    collection
}

pub fn get_people(group: String) -> Vec<String> {
    let collection: Vec<String> = group.split("\n").map(|x| x.to_string()).collect();
    collection
}

pub fn score_group(group: String) -> usize {
    let people = get_people(group);

    let answers = people.iter().fold(String::new(), |mut collection, person| {
        collection += &person;
        collection
    });

    let score: HashSet<_> = answers.chars().collect();

    score.len()
}

pub fn score_group2(group: String) -> usize {
    let people = get_people(group);

    let answers = people.iter().fold(Vec::new(), |mut collection, person| {
        collection.push(person.chars().collect::<HashSet<_>>());
        collection
    });

    let group_answer = answers
        .iter()
        .fold(answers[0].clone(), |mut collection, answer| {
            collection = collection.intersection(&answer).copied().collect();
            collection
        });

    group_answer.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_example() {
        let groups = read_groups("example1.txt");
        assert_eq!(groups.len(), 5);

        let score = groups.iter().fold(0, |mut score, group| {
            score += score_group(group.to_string());
            score
        });

        assert_eq!(score, 11);
    }

    #[test]
    fn part_1_solution() {
        let groups = read_groups("input.txt");

        let score = groups.iter().fold(0, |mut score, group| {
            score += score_group(group.to_string());
            score
        });

        assert_eq!(score, 6703);
    }

    #[test]
    fn part_2_example() {
        let groups = read_groups("example1.txt");
        assert_eq!(groups.len(), 5);

        let score = groups.iter().fold(0, |mut score, group| {
            score += score_group2(group.to_string());
            score
        });

        assert_eq!(score, 6);
    }

    #[test]
    fn part_2_solution() {
        let groups = read_groups("input.txt");

        let score = groups.iter().fold(0, |mut score, group| {
            score += score_group2(group.to_string());
            score
        });

        assert_eq!(score, 3430);
    }
}
