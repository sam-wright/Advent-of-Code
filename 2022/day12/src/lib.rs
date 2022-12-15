use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

type Map = HashMap<(usize, usize), u8>;

//
// Read in map a-z => 97-122
//
pub fn read_input(filename: &str) -> (Map, (usize, usize), (usize, usize)) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut map = HashMap::new();
    let mut start = (0, 0);
    let mut finish = (0, 0);
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.as_bytes().iter().enumerate() {
            if c == &83 {
                // "S"
                map.insert((x, y), 97); // "a"
                start = (x, y);
            } else if c == &69 {
                // "E"
                map.insert((x, y), 122); //"z"
                finish = (x, y);
            } else {
                map.insert((x, y), *c);
            }
        }
    }
    (map, start, finish)
}

fn get_actions(map: &Map, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut actions = Vec::new();
    let h = map[pos];
    let mut dirs = vec![(pos.0, pos.1 + 1), (pos.0 + 1, pos.1)];
    if pos.0 > 0 {
        dirs.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        dirs.push((pos.0, pos.1 - 1));
    }

    for d in dirs {
        if let Some(p) = map.get(&d) {
            if p <= &(h + 1) {
                actions.push(d);
            }
        }
    }

    actions
}
fn get_actions_descent(map: &Map, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut actions = Vec::new();
    let h = map[pos];
    let mut dirs = vec![(pos.0, pos.1 + 1), (pos.0 + 1, pos.1)];
    if pos.0 > 0 {
        dirs.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        dirs.push((pos.0, pos.1 - 1));
    }

    for d in dirs {
        if let Some(p) = map.get(&d) {
            if p >= &(h - 1) {
                actions.push(d);
            }
        }
    }

    actions
}

pub fn explore(map: &Map, start: (usize, usize), finish: (usize, usize)) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((vec![start], 0));
    let mut visited = HashSet::new();

    loop {
        if queue.is_empty() {
            break;
        }

        let (path, _) = queue.pop_front().unwrap();
        let pos = path.last().unwrap();

        if visited.contains(pos) {
            continue;
        }
        visited.insert(*pos);
        let actions = get_actions(map, pos);

        for action in actions {
            if action == finish {
                return path.len();
            }
            if path.contains(&action) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(action);
            let cost = path.len();

            let idx = queue.partition_point(|&(_, x)| x < cost);
            queue.insert(idx, (new_path, cost));
        }
    }
    0
}

pub fn descend(map: &Map, start: (usize, usize)) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((vec![start], 0));
    let mut visited = HashSet::new();

    loop {
        if queue.is_empty() {
            break;
        }

        let (path, _) = queue.pop_front().unwrap();
        let pos = path.last().unwrap();

        if visited.contains(pos) {
            continue;
        }
        visited.insert(*pos);
        let actions = get_actions_descent(map, pos);

        for action in actions {
            if map[&action] == b'a' {
                return path.len();
            }
            if path.contains(&action) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(action);
            let cost = path.len();

            let idx = queue.partition_point(|&(_, x)| x < cost);
            queue.insert(idx, (new_path, cost));
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let (map, start, finish) = read_input("example.txt");
        assert_eq!(start, (0, 0));
        assert_eq!(finish, (5, 2));
        assert_eq!(map[&start], 'a' as u8);
        assert_eq!(map[&finish], 'z' as u8);

        assert_eq!(explore(&map, start, finish), 31);
    }

    #[test]
    fn part1() {
        let (map, start, finish) = read_input("input.txt");

        assert_eq!(explore(&map, start, finish), 528);
    }

    #[test]
    fn example2() {
        let (map, _, finish) = read_input("example.txt");

        assert_eq!(descend(&map, finish), 29);
    }

    #[test]
    fn part2() {
        let (map, _, finish) = read_input("input.txt");

        assert_eq!(descend(&map, finish), 522);
    }
}
