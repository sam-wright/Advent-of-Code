use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
extern crate regex;
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

        let a = instructions.entry(name).or_insert_with(Thing::new);
        a.depends.push(depends);
        a.complete = false;

        instructions.entry(depends).or_insert_with(Thing::new);

        println!("{:?}", line);
    }

    print!("\n");
    for i in &instructions {
        println!("{:?}", &i);
    }
    let mut entry = 'A';

    let mut completion_order = Vec::with_capacity(100);

    completion_order.push('C');

    for _ in 0..100 {
        for (me, this) in &instructions {
            // for each instruction step

            // make sure we haven't already completed this step
            if completion_order.contains(&*me) {
                continue;
            }

            let mut complete = true;
            for d in &this.depends {
                // check each dependancy
                if !instructions.get(&d).expect("missing dependancy").complete {
                    complete = false;
                    break;
                }
            }
            entry = *me;
            if complete {
                // if its dependancies are met,
                // go mark it complete
                completion_order.push(*me);
                println!("pushing {}", &me);
                break;
            }
        }

        instructions
            .get_mut(&entry)
            .expect("missing dependancy")
            .complete = true;
    }

    for c in &completion_order {
        print!("{}", c);
    }
    Ok(())
}
