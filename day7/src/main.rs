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
    fn new() -> Thing {
        Thing {
            depends: Vec::new(),
            //depends: depends,
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

    let mut instructions = HashMap::new();
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

        //instructions.insert(name, Thing::new(depends));
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

    let mut running = true;
    while running {
        running = false;

        let steps = instructions.iter();
        for (i,  step) in steps {
            // for each instruction
            let mut this = instructions.get(&i).expect("could not get this");
            for d in &this.depends {
                let dep = instructions.get(&d).expect("missing dpendancy");

                if !dep.complete {
                    break;
                }
            }
            step.complete = true;
        }
    }
    Ok(())
}
