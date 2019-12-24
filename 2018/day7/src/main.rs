use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
extern crate regex;
use petgraph::algo::toposort;
use petgraph::algo::DfsSpace;
use petgraph::visit::Dfs;
use petgraph::visit::DfsPostOrder;
use petgraph::Graph;
use regex::Regex;

#[derive(Debug)]
struct Thing {
    depends: Vec<char>,
    complete: bool,
}

impl Thing {
    fn new() -> Self {
        Thing {
            depends: Vec::new(),
            complete: true,
        }
    }
}

fn main() -> io::Result<()> {
    //let mut file = File::open("input.txt")?;
    let mut file = File::open("test_input.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.replace(",", "");
    let collection: Vec<&str> = contents[..contents.len() - 1].split('\n').collect();

    let name_re = Regex::new(r"step (\w)").expect("Regex creation failed");
    let depends_re = Regex::new(r"Step (\w)").expect("Regex creation failed");

    let mut instructions: HashMap<char, Thing> = HashMap::new();
    let mut graph = Graph::<char, i32>::new();
    for line in collection {
        let name = name_re
            .captures(&line)
            .expect("Unable to match name regex")
            .get(1)
            .map_or(0 as char, |v| v.as_str().chars().next().expect(""));

        let depends = depends_re
            .captures(&line)
            .expect("Unable to match depends regex")
            .get(1)
            .map_or(0 as char, |v| v.as_str().chars().next().expect(""));

        let n = match graph
            .node_indices()
            .find(|f| graph.node_weight(*f) == Some(&name))
        {
            Some(v) => v,
            None => graph.add_node(name),
        };

        let d = match graph
            .node_indices()
            .find(|f| graph.node_weight(*f) == Some(&depends))
        {
            Some(v) => v,
            None => graph.add_node(depends),
        };
        graph.add_edge(d, n, 1);

        let a = instructions.entry(name).or_insert_with(Thing::new);
        a.depends.push(depends);
        a.complete = false;

        instructions.entry(depends).or_insert_with(Thing::new);

        println!("{:?}", line);
    }

    println!("{:?}", &graph);
    print!("\n");
    for i in &instructions {
        println!("{:?}", &i);
    }

    let mut space = toposort(&graph, None).expect("Unable to sort");
    println!("{:?}", space);
    //space.reverse();
    for s in &space {
        print!("{}", graph.node_weight(*s).expect("could not get"));
    }
    println!("\n");

    let mut dfs = Dfs::new(&graph, *space.first().expect(""));
    while let Some(nx) = dfs.next(&graph) {
        print!("{:?}", graph.node_weight(nx).expect("could not get"));
    }

    println!("\n");

    Ok(())
}
