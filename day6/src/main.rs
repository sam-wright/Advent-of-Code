use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Read};
use std::iter::FromIterator;

struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord { x: x, y: y }
    }

    fn get_distance(&self, x: i32, y: i32) -> i32 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

fn count(val: i32, range: &[i32]) -> i32 {
    range
        .iter()
        .fold(0i32, |count, v| if v == &val { count + 1 } else { count })
}

fn is_unique(dists: &[i32]) -> bool {
    let hs: HashSet<&i32> = HashSet::from_iter(dists.iter().clone());
    hs.len() == dists.len()
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    //let mut file = File::open("test_input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.replace(",", "");
    let collection: Vec<&str> = contents[..contents.len() - 1].split('\n').collect();

    let mut coords = Vec::new();

    // Construct the Coords
    for line in &collection {
        let pts: Vec<&str> = line.split_whitespace().collect();
        // println!("{:?}", &pts);
        coords.push(Coord::new(
            pts[0].parse().expect("Unable to make X number good"),
            pts[1].parse().expect("Unable to make Y number good"),
        ));
    }

    // println!("{:?}", collection);
    // println!("{}", coords.len());

    // Get maxX and maxY
    let max_x = coords
        .iter()
        .fold(0i32, |max_x, v| if v.x > max_x { v.x } else { max_x });
    let max_y = coords
        .iter()
        .fold(0i32, |max_y, v| if v.y > max_y { v.y } else { max_y });

    // Now calculate the minimal grid
    let mut minimal_grid = HashMap::new();
    let mut count_within_10k = 0;
    for y in 0..=max_y {
        for x in 0..=max_x {
            let dists: Vec<i32> = coords.iter().fold(Vec::new(), |mut dists, coord| {
                dists.push(coord.get_distance(x, y));
                dists
            });
            let min_dist = dists.iter().min().expect("unable to find min");
            let min_index = dists.iter().position(|&r| r == *min_dist).unwrap();
            let sum_dist = dists.iter().fold(0i32, |sum, &v| sum + v);
            let g = minimal_grid.entry(min_index).or_insert(0);

            if count(*min_dist, &dists) > 1 {
                // print!(". ");
            } else {
                // print!("{} ", min_index);
                *g += 1;
            }
            //if sum_dist <= 32 && is_unique(&dists) {
            if sum_dist < 10000 {
                count_within_10k += 1;
            }
        }
        // print!("\n");
    }

    // print!("\n");

    // Calculate a slightly larget grid
    let mut bigger_grid = HashMap::new();
    for y in -1..=max_y + 1 {
        for x in -1..=max_x + 1 {
            let dists: Vec<i32> = coords.iter().fold(Vec::new(), |mut dists, coord| {
                dists.push(coord.get_distance(x, y));
                dists
            });
            let min_dist = dists.iter().min().expect("unable to find min");
            let min_index = dists.iter().position(|&r| r == *min_dist).unwrap();

            let g = bigger_grid.entry(min_index).or_insert(0);

            if count(*min_dist, &dists) > 1 {
                // print!(". ");
            } else {
                // print!("{} ", min_index);
                *g += 1;
            }
        }
        // print!("\n");
    }

    // Compare the two grids
    // println!("\n{:?}", minimal_grid);
    // println!("{:?}", bigger_grid);

    let mut max_area = 0i32;
    let mut max_index = -1i32;
    for i in 0..coords.len() {
        let g1 = bigger_grid.entry(i).or_insert(0);
        let g2 = minimal_grid.entry(i).or_insert(0);

        if *g1 != *g2 {
            continue;
        }
        if *g1 > max_area {
            max_area = *g1;
            max_index = i as i32;
        }
    }
    println!("Max Index: {}\nMax Area: {}", &max_index, &max_area);
    println!("Area within 10k: {}", &count_within_10k);
    Ok(())
}
