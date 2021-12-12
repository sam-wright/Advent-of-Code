use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

type Node = String;
type Edge = (Node, Node);

pub fn read_input(filename: &str) -> Vec<Edge> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|x| {
            let node_string = x.unwrap();
            let mut nodes = node_string.split('-');
            (
                nodes.next().unwrap().to_string(),
                nodes.next().unwrap().to_string(),
            )
        })
        .collect()
}

fn find_expansions(edges: &Vec<Edge>, node: &Node) -> Vec<Node> {
    // oops this is a unidirectional expansion, we dont want that...
    edges
        .iter()
        .filter(|(x, y)| x == node || y == node)
        .map(|(x, y)| {
            if x == node {
                y.to_string()
            } else {
                x.to_string()
            }
        })
        .collect()
}

pub fn explore(edges: &Vec<Edge>) -> i32 {
    let mut queue = Vec::new();

    queue.push(vec!["start".to_string()]);

    let mut paths = 0;
    while !queue.is_empty() {
        let path = queue.pop().unwrap();
        let expansion_node = path.last().unwrap();

        if expansion_node == "end" {
            paths += 1;
            continue;
        }

        // get possible expansions
        let expansions = find_expansions(edges, &expansion_node);

        for expansion in &expansions {
            // Dont explore small caves multiple times
            let first_char = expansion.chars().into_iter().next().unwrap();
            if first_char.is_lowercase() && path.contains(expansion) {
                continue;
            }

            // println!("Expanding: {:?} -> {:?}", path, &expansion);
            let mut new_path = path.clone();
            new_path.push(expansion.to_string());

            queue.push(new_path);
        }
    }

    paths
}

fn double_exploration(path: &Vec<Node>) -> bool {
    let small_caves: Vec<&Node> = path
        .iter()
        .filter(|x| x.chars().into_iter().next().unwrap().is_lowercase())
        .collect();
    let set: HashSet<&&Node> = small_caves.iter().collect();
    set.len() != small_caves.len()
}

pub fn explore_revised(edges: &Vec<Edge>) -> i32 {
    let mut queue = Vec::new();

    queue.push(vec!["start".to_string()]);

    let mut paths = 0;
    while !queue.is_empty() {
        let path = queue.pop().unwrap();
        let expansion_node = path.last().unwrap();

        if expansion_node == "end" {
            paths += 1;
            continue;
        }

        // get possible expansions
        let expansions = find_expansions(edges, &expansion_node);

        for expansion in &expansions {
            // Dont _ever_ explore start
            if expansion == "start" {
                continue;
            }

            // Allow a single double-exploration now
            if double_exploration(&path) {
                // Dont explore small caves multiple times
                let first_char = expansion.chars().into_iter().next().unwrap();
                if first_char.is_lowercase() && path.contains(expansion) {
                    continue;
                }
            }

            // println!("Expanding: {:?} -> {:?}", path, &expansion);
            let mut new_path = path.clone();
            new_path.push(expansion.to_string());

            queue.push(new_path);
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let edges1 = read_input("example1.txt");
        let paths1 = explore(&edges1);
        assert_eq!(paths1, 10);

        let edges2 = read_input("example2.txt");
        let paths2 = explore(&edges2);
        assert_eq!(paths2, 19);

        let edges3 = read_input("example3.txt");
        let paths3 = explore(&edges3);
        assert_eq!(paths3, 226);
    }

    #[test]
    fn part1() {
        let edges = read_input("input.txt");
        let paths = explore(&edges);
        assert_eq!(paths, 4338);
    }

    #[test]
    fn example2() {
        let edges1 = read_input("example1.txt");
        let paths1 = explore_revised(&edges1);
        assert_eq!(paths1, 36);

        let edges2 = read_input("example2.txt");
        let paths2 = explore_revised(&edges2);
        assert_eq!(paths2, 103);

        let edges3 = read_input("example3.txt");
        let paths3 = explore_revised(&edges3);
        assert_eq!(paths3, 3509);
    }

    #[test]
    fn part2() {
        let edges = read_input("input.txt");
        let paths = explore_revised(&edges);
        assert_eq!(paths, 114189);
    }
}
