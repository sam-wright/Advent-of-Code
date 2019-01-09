use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

struct Coord {
    x: i32,
    y: i32,
    area: Option<i32>,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord {
            x: x,
            y: y,
            area: None,
        }
    }

    fn get_distance(&self, x: i32, y: i32) -> i32 {
        (self.x - x).abs() + (self.y - y)
    }
}

fn main() -> io::Result<()> {
    //let mut file = File::open("input.txt")?;
    let mut file = File::open("test_input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.replace(",", "");
    let collection: Vec<&str> = contents[..contents.len() - 1].split('\n').collect();

    let mut coords = Vec::new();

    // Construct the Coords
    for line in &collection {
        let pts: Vec<&str> = line.split_whitespace().collect();
        println!("{:?}", &pts);
        coords.push(Coord::new(
            pts[0].parse().expect("Unable to make X number good"),
            pts[1].parse().expect("Unable to make Y number good"),
        ));
    }

    println!("{:?}", collection);
    println!("{}", coords.len());

    // Get maxX and maxY
    let max_x = coords
        .iter()
        .fold(0i32, |max_x, v| if v.x > max_x { v.x } else { max_x });
    let max_y = coords
        .iter()
        .fold(0i32, |max_y, v| if v.y > max_y { v.y } else { max_y });

    // Now calculate the grid
    let mut grid = HashMap::new();
    for x in 0..max_x {
        for y in 0..max_y {
            let mut dists: Vec<i32> = coords.iter().fold(Vec::new(), |mut dists, coord| {
                dists.push(coord.get_distance(x, y));
                dists
            });
            // let g = grid.entry((x,y)).or_insert(0);
            // *g = dists.iter().enumerate().fold(1000, |min, dist, i|)
        }
    }

    Ok(())
}
