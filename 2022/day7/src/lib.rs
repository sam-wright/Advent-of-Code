use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
pub struct Node {
    filename: String,
    children: Vec<Node>,
    filesize: i32,
}

impl Node {
    fn new(filename: &str) -> Self {
        let filename = filename.to_string();
        Node {
            filename,
            children: Vec::new(),
            filesize: 0,
        }
    }
    fn new_with_size(filename: &str, filesize: i32) -> Self {
        let filename = filename.to_string();

        Node {
            filename,
            children: Vec::new(),
            filesize,
        }
    }
}

pub fn read_input(filename: &str) -> Node {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut filesystem = Node::new("/");

    let mut cwd = &mut filesystem;
    let mut location: Vec<String> = Vec::new();
    // let mut location = "/".to_string();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("$ cd /") {
            // just ignore
        } else if line.contains("$ cd") {
            // move the cwd pointer
            let target = line.replace("$ cd ", "");
            if target == ".." {
                location.pop();
                cwd = &mut filesystem;

                for l in &location {
                    cwd = cwd
                        .children
                        .iter_mut()
                        .find(|child| child.filename == *l)
                        .unwrap();
                }
            } else {
                location.push(target.clone());

                cwd = cwd
                    .children
                    .iter_mut()
                    .find(|child| child.filename == target)
                    .unwrap();
            }
        } else if line.contains("$ ls") {
            // just ignore
        } else {
            // get the contents of an ls command
            let mut s = line.split(' ');
            let size: i32 = s.next().unwrap().parse().unwrap_or(0);
            let name = s.next().unwrap();

            let new_file = Node::new_with_size(name, size);
            cwd.children.push(new_file);

            // // update ancestor dir filesizes
            // unsafe {
            // ...
            //  mutable graphs with ownership is hard
            // ...
            //     }
            // }
        }
    }
    filesystem
}

pub fn get_size(fs: &Node) -> (i32, i32) {
    // the size of yourself
    let mut size = fs.filesize;
    let mut acc = 0;
    // and the size of your children
    for child in &fs.children {
        let s = get_size(child);
        size += s.1;
        acc += s.0;
    }

    if !fs.children.is_empty() {
        println!("{}: {}", fs.filename, size);

        if size <= 100000 {
            acc += size;
        }
    }
    (acc, size)
}

pub fn get_sizes(fs: &Node, list: &mut Vec<i32>) -> (i32, i32) {
    // the size of yourself
    let mut size = fs.filesize;
    let mut acc = 0;
    // and the size of your children
    for child in &fs.children {
        let s = get_sizes(child, list);
        size += s.1;
        acc += s.0;
    }

    if !fs.children.is_empty() {
        list.push(size);
        if size <= 100000 {
            acc += size;
        }
    }
    (acc, size)
}

pub fn find_best_dir(target: i32, list: &Vec<i32>) -> i32 {
    let mut r = 70000000;
    let mut s = 0;

    for l in list {
        if l > &target && l - target < r {
            r = l - target;
            s = *l;
        }
    }
    s
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let filesystem = read_input("example.txt");
        assert_eq!(get_size(&filesystem).0, 95437);
    }

    #[test]
    fn part1() {
        let filesystem = read_input("input.txt");

        assert_eq!(get_size(&filesystem).0, 1770595);
        //5772414 Too high
    }

    #[test]
    fn example2() {
        let filesystem = read_input("example.txt");
        let size = get_size(&filesystem).1;
        let target = 30000000 - (70000000 - size);
        println!("Need to delete at least {}", target);

        let mut list = Vec::new();
        get_sizes(&filesystem, &mut list);

        assert_eq!(find_best_dir(target, &list), 24933642);
    }

    #[test]
    fn part2() {
        let filesystem = read_input("input.txt");
        let size = get_size(&filesystem).1;
        let target = 30000000 - (70000000 - size);
        println!("Need to delete at least {}", target);

        let mut list = Vec::new();
        get_sizes(&filesystem, &mut list);

        assert_eq!(find_best_dir(target, &list), 2195372);
    }
}
