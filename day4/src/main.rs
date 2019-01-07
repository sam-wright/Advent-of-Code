extern crate chrono;
use chrono::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashMap;

use std::fs::File;
use std::io::{self, Read};
extern crate regex;
use regex::Regex;

fn record_time(record: &mut HashMap<u32, [u32; 60]>, guard: u32, sleep_time: u32, wake_time: u32) {
    let entry = record.entry(guard).or_insert([0; 60]);
    for i in sleep_time..wake_time {
        entry[i as usize] += 1;
    }
}

fn main() -> io::Result<()> {
    //let mut file = File::open("input.txt")?;
    let mut file = File::open("test_input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let collection: Vec<&str> = contents.split('\n').collect();

    let mut active_guard = 0;
    let mut last_asleep = 0;
    let mut last_wake = 0;

    let guard_re = Regex::new(r"\#(\d*)").expect("Regex creation failed");
    let time_re = Regex::new(r"\[(.*)\](.*)").expect("Regex creation failed");
    let sleep_re = Regex::new(r"asleep").expect("Regex creation failed");
    let wake_re = Regex::new(r"wakes").expect("Regex creation failed");

    let mut btm = BTreeMap::new();
    let mut record = HashMap::new();

    for line in &collection {
        let time_str = match time_re.captures(&line) {
            Some(time) => time.get(1).map_or("", |v| v.as_str()),
            None => continue,
        };
        let time =
            NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M").expect("incorrect format");
        let rest = match time_re.captures(&line) {
            Some(rest) => rest.get(2).map_or("", |v| v.as_str()),
            None => continue,
        };
        btm.entry(time).or_insert(rest);
    }

    for (key, value) in &btm {
        active_guard = match guard_re.captures(&value) {
            Some(guard) => guard
                .get(1)
                .map_or(0, |v| v.as_str().parse().expect("Expected an number")),
            None => active_guard,
        };

        if sleep_re.is_match(&value) {
            last_asleep = key.minute();
        }
        if wake_re.is_match(&value) {
            last_wake = key.minute();

            record_time(&mut record, active_guard, last_asleep, last_wake);
        }
    }

    let mut most_slept_guard = 0;
    let mut most_slept_amount = 0;
    for (k, v) in &record {
        let sleep_amount = v.into_iter().sum::<u32>();

        if sleep_amount > most_slept_amount {
            most_slept_amount = sleep_amount;
            most_slept_guard = *k
        }
    }

    let max_time_value = record
        .get(&most_slept_guard)
        .expect("Unable to find guard")
        .iter()
        .max()
        .expect("");

    let max_time_index = record
        .get(&most_slept_guard)
        .expect("Unable to find guard")
        .iter()
        .enumerate()
        .fold(
            0,
            |max, (i, val)| if val == max_time_value { i } else { max },
        );

    println!(
        "Part1 - Max sleep time: {}\n                 Guard: {}\n                        { }\n",
        max_time_index,
        most_slept_guard,
        most_slept_guard as usize * max_time_index
    );

    // Part 2
    let mut total_minutes = [0u32; 60];
    for (_guard, minutes) in &record {
        for i in 0..60 {
            total_minutes[i] += minutes[i];
        }
    }

    let mut most_slept_total = 0;
    let most_slept_minute = total_minutes.iter().enumerate().fold(0, |max, (i, val)| {
        if val > &most_slept_total {
            most_slept_total = *val;
            i
        } else {
            max
        }
    });

    let mut most_slept_at_minute = 0;
    let most_slept_guard = record.iter().fold(0, move |most, val| {
        if val.1[most_slept_minute as usize] > most_slept_at_minute {
            most_slept_at_minute = val.1[most_slept_minute as usize];
            *val.0
        } else {
            most
        }
    });

    println!(
        "Part2 - Most Slept Minute: {}\n         Most Slept Guard: {}\n                          {}",
        most_slept_minute,
        most_slept_guard,
        most_slept_minute * most_slept_guard as usize
    );

    Ok(())
}
