use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    //let mut file = File::open("test_input.txt")?;
    //let mut file = File::open("test_input2.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let collection: Vec<&str> = contents[..contents.len() - 1].split('\n').collect();
    let letters = vec![
        'a', 'b', 'a', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
        'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut hash = HashMap::new();
    for line in &collection {
        println!("{} --", &line);

        let mut double = false;
        let mut triple = false;

        for letter in &letters {
            let n = line.matches(*letter).count();

            //println!("\t {} {}", *letter, n);

            match n {
                0 => {}
                1 => {}
                2 => double = true,
                3 => triple = true,
                _ => println!("need more buckets!"),
            };
        }
        if double {
            let counter = hash.entry(2).or_insert(0);
            *counter += 1;
        }
        if triple {
            let counter = hash.entry(3).or_insert(0);
            *counter += 1;
        }
    }

    println!("\nord\tinst");
    let mut total_hash = 1;
    for val in 2..4 {
        let x = hash.entry(val).or_insert(0);

        println!("{}\t{}", val, *x);

        total_hash *= *x;
    }

    println!("total_hash = {}", total_hash);

    // Part-2
    for line in &collection {
        for compline in &collection {
            let mut resline = String::from("");
            for i in 0..line.len() {
                if line.as_bytes()[i] == compline.as_bytes()[i] {
                    resline.push_str(&line[i..=i]);
                }
            }

            if resline.len() + 1 == line.len() {
                println!(
                    "I think we have a match!!\nline\t--\t{}\ncmpline\t--\t{}\nresline\t--\t{}",
                    line, compline, resline
                );
            }
        }
    }

    Ok(())
}
