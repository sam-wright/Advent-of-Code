use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    //let mut file = File::open("test_input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    contents = contents.replace("+", "");
    let collection: Vec<&str> = contents.split('\n').collect();
    let numbers: Vec<i64> = collection[..collection.len() - 1]
        .iter()
        .map(|v| v.parse().expect("expected a number"))
        .collect();
    // For some reason the last element of collection is empty, hence processing up to len-1

    let mut temp_sum = 0;
    let mut frequencies = HashMap::new();
    let mut looking_for_dup = true;
    let mut first_run = true;

    let mut sum = 0i64;
    // Part1
    while looking_for_dup {
        sum = numbers.iter().fold(sum, |sum, &val| {
            temp_sum = sum + val;
            //println!("temp sum = {}", temp_sum);
            let counter = frequencies.entry(temp_sum).or_insert(0);
            *counter += 1;

            //println!("counter = {}", counter);

            if *counter > 1 {
                println!("Found dup = {}", temp_sum);

                looking_for_dup = false;
            }

            temp_sum
        });

        if first_run {
            println!("Final sum = {}", sum);
            first_run = false;
        }
    }
    // Part2

    Ok(())
}
