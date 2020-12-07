// #![recursion_limit = "1023"]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

pub fn read_rules(filename: &str) -> Vec<String> {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let collection: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    collection
}

pub fn extract_rule(rule: &str) -> (String, Vec<(i32, String)>) {
    let rule = &rule.replace(" bags", "");
    let rule = &rule.replace(" contain", "");
    let rule = &rule.replace(" bag", "");
    let rule = &rule.replace(",", "");
    let rule = &rule.replace(".", "");

    let words: Vec<String> = rule.split(" ").map(|x| x.to_string()).collect();

    let parent_node = String::from(&words[0]) + " " + &words[1];

    let mut nodes = Vec::new();
    for i in (2..words.len()).step_by(3) {
        if words[i] == "no" {
            continue;
        }
        nodes.push((
            words[i].parse::<i32>().unwrap(),
            String::from(&words[i + 1]) + " " + &words[i + 2],
        ));
    }

    (parent_node, nodes)
}

fn get_parents(target: &str, rules_engine: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut new_parents = Vec::new();
    for (parent, children) in rules_engine {
        for child in children {
            if child == target {
                new_parents.push(parent.clone());
            }
        }
    }

    new_parents
}

fn count_parents(finish: &str, rules_engine: &HashMap<String, Vec<String>>) -> usize {
    let mut parents: HashSet<String> = HashSet::new();

    // Seed the exploration
    for np in get_parents(finish, rules_engine) {
        parents.insert(np);
    }

    // while there are new parents discovered, keep searching (expensivly)
    // otherwise return the number of parents
    let mut search = true;
    while search {
        search = false;
        let mut new_parents = Vec::new();
        for p in &parents {
            new_parents.append(&mut get_parents(p, rules_engine));
        }

        for np in new_parents {
            search |= parents.insert(np);
        }
    }

    parents.len()
}

fn explore(current_node: &str, rules_engine: &HashMap<String, Vec<(i32, String)>>) -> i32 {
    let mut score = 1;
    match rules_engine.get(current_node) {
        Some(children) => {
            for (cost, child) in children {
                score += cost * explore(&child, rules_engine);
            }
        }
        None => {}
    }
    score
}

pub fn count_the_ways_rev(rules: Vec<String>, target: &str) -> usize {
    let mut rules_engine: HashMap<String, Vec<String>> = HashMap::new();

    for rule_str in &rules {
        let (parent, children) = extract_rule(&rule_str);

        for (_cost, child) in children {
            let e = rules_engine
                .entry(parent.clone())
                .or_insert(vec![child.clone()]);
            e.push(child.clone());
        }
    }

    count_parents(target, &rules_engine)
}

pub fn count_capacity(rules: Vec<String>, target: &str) -> i32 {
    let mut rules_engine: HashMap<String, Vec<(i32, String)>> = HashMap::new();

    for rule_str in &rules {
        let (parent, children) = extract_rule(&rule_str);

        for (cost, child) in children {
            let e = rules_engine.entry(parent.clone()).or_insert(Vec::new());
            e.push((cost, child.clone()));
        }
    }

    count_weighted_children(target, &rules_engine) - 1
}

fn count_weighted_children(start: &str, rules_engine: &HashMap<String, Vec<(i32, String)>>) -> i32 {
    explore(start, rules_engine)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1_example() {
        let rules = read_rules("example1.txt");
        let score = count_the_ways_rev(rules, "shiny gold");
        assert_eq!(score, 4);
    }

    #[test]
    fn rule_extractor_test() {
        let rule_str = "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.";
        let rule = extract_rule(rule_str);
        assert_eq!(rule.0, "vibrant plum");
        assert_eq!(rule.1.len(), 2);

        let rule_str = "vibrant plum bags contain 5 faded blue bags.";
        let rule = extract_rule(rule_str);
        assert_eq!(rule.0, "vibrant plum");
        assert_eq!(rule.1.len(), 1);

        let rule_str = "faded yellow bags contain 3 posh lime bags, 4 wavy blue bags, 3 faded crimson bags, 2 shiny lavender bags.";
        let rule = extract_rule(rule_str);
        assert_eq!(rule.0, "faded yellow");
        assert_eq!(rule.1.len(), 4);
    }

    #[test]
    fn part_1_solution() {
        let rules = read_rules("input.txt");
        let score = count_the_ways_rev(rules, "shiny gold");

        assert_eq!(score, 172)
    }

    #[test]
    fn part_2_example_1() {
        let rules = read_rules("example1.txt");
        let score = count_capacity(rules, "shiny gold");

        assert_eq!(score, 32);
    }

    #[test]
    fn part_2_example_2() {
        let rules = read_rules("example2.txt");
        let score = count_capacity(rules, "shiny gold");
        assert_eq!(score, 126);
    }

    #[test]
    fn part_2_solution() {
        let rules = read_rules("input.txt");
        let score = count_capacity(rules, "shiny gold");
        assert_eq!(score, 39645);
    }
}
