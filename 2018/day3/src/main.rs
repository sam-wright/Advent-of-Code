use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

fn record_claim(
    map: &mut HashMap<(u32, u32), u32>,
    x_start: u32,
    y_start: u32,
    x_size: u32,
    y_size: u32,
) {
    for x in 0..x_size {
        for y in 0..y_size {
            let m = map.entry((x_start + x, y_start + y)).or_insert(0);
            *m += 1;
        }
    }
}

fn check_claim(
    map: &mut std::collections::HashMap<(u32, u32), u32>,
    x_start: u32,
    y_start: u32,
    x_size: u32,
    y_size: u32,
) -> bool {
    for x in 0..x_size {
        for y in 0..y_size {
            let m = map.entry((x_start + x, y_start + y)).or_insert(0);
            if *m > 1 {
                return false;
            }
        }
    }
    true
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    //let mut file = File::open("test_input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.replace(',', " ");
    contents = contents.replace('x', " ");

    contents = contents.replace(':', "");
    contents = contents.replace("@ ", "");
    contents = contents.replace('#', "");

    let collection: Vec<&str> = contents.split('\n').collect();

    let mut map: HashMap<(u32, u32), u32> = HashMap::new();

    for c in &collection {
        if c == &"" {
            continue;
        }
        println!("{}", &c);
        let line_strs: Vec<&str> = c.split(' ').collect();
        let line_data: Vec<u32> = line_strs
            .iter()
            .map(|v| v.parse().expect("expected a number"))
            .collect();

        record_claim(
            &mut map,
            line_data[1],
            line_data[2],
            line_data[3],
            line_data[4],
        );
    }
    let mut conflicts = 0;
    for (pos, &inst) in &map {
        if inst > 1 {
            println!("Conflict in {}x{}", pos.0, pos.1);
            conflicts += 1;
        }
    }

    println!("Map len = {}", conflicts);

    for c in &collection {
        if c == &"" {
            continue;
        }
        println!("{}", &c);
        let line_strs: Vec<&str> = c.split(' ').collect();
        let line_data: Vec<u32> = line_strs
            .iter()
            .map(|v| v.parse().expect("expected a number"))
            .collect();

        if check_claim(
            &mut map,
            line_data[1],
            line_data[2],
            line_data[3],
            line_data[4],
        ) {
            println!("Found the unconflicted claim!  {}", line_data[0]);
            break;
        }
    }
    Ok(())
}
