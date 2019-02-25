use std::fs::File;
use std::io::{self, Read};

#[derive(Default, Debug)]
struct Node {
    child_nodes: Vec<Node>,
    metadata_entries: Vec<i32>,
    num_child_nodes: i32,
    num_metadata: i32,
}

impl Node {
    fn new(buffer: &[i32], mut index: i32) -> (i32, Self) {
        let mut new_node = Self {
            ..Default::default()
        };

        new_node.num_child_nodes = *buffer
            .get(0)
            .expect("Unable to get num_children from buffer");
        new_node.num_metadata = *buffer
            .get(1)
            .expect("Unable to get num_metadata from buffer");
        index += 2;

        for _child in 0..new_node.num_child_nodes {
            let (updated_index, child_node) = Node::new(
                &buffer
                    .get(index as usize..)
                    .expect("Failed to slice buffer"),
                0,
            );
            new_node.child_nodes.push(child_node);
            index += updated_index;
        }

        for i in 0..new_node.num_metadata {
            new_node.metadata_entries.push(
                *buffer
                    .get(index as usize + i as usize)
                    .expect("Failed to get metadata from buffer"),
            );
        }
        index += new_node.num_metadata;

        (index, new_node)
    }

    fn collect_metadata(&self) -> i32 {
        let sum: i32 = self.metadata_entries.iter().sum();

        sum + self
            .child_nodes
            .iter()
            .fold(0i32, |sum, v| sum + v.collect_metadata())
    }

    fn compute_value(&self) -> i32 {
        let mut value = 0i32;

        if self.num_child_nodes > 0 {
            for entry in &self.metadata_entries {
                if *entry == 0 {
                    continue;
                }
                value += match self.child_nodes.get(*entry as usize - 1) {
                    Some(v) => v.compute_value(),
                    None => 0,
                };
            }
        } else {
            value = self.metadata_entries.iter().sum();
        }
        value
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    //let mut file = File::open("test_input.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.replace(",", "");
    let collection: Vec<i32> =
        contents[..contents.len() - 1]
            .split(' ')
            .fold(Vec::new(), |mut v, val| {
                v.push(val.parse().expect("unable to parse i32"));
                v
            });

    let (_, root_node) = Node::new(&collection, 0);

    dbg!(&root_node);

    println!("Collected metadata: {}\n", root_node.collect_metadata());
    println!("\nRoot Value: {}", root_node.compute_value());

    Ok(())
}
