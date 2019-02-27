extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
struct Observation {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Observation {
    fn new(x: i32, y: i32, dx: i32, dy: i32) -> Self {
        Observation { x, y, dx, dy }
    }
}

fn predict(observations: &[Observation], t: i32) -> Vec<Observation> {
    observations.iter().fold(Vec::new(), |mut v, observation| {
        v.push(Observation::new(
            (observation.x) + observation.dx * t,
            (observation.y) + observation.dy * t,
            observation.dx,
            observation.dy,
        ));

        v
    })
}

fn print_grid(observations: &[Observation], offsets: (i32, i32)) {
    let (max_x, max_y) = observations
        .iter()
        .fold((0i32, 0i32), |(mut max_x, mut max_y), v| {
            if v.x > max_x {
                max_x = v.x;
            }
            if v.y > max_y {
                max_y = v.y;
            }
            (max_x, max_y)
        });

    dbg!((max_x, max_y));
    let mut grid = Box::new([[32u8; 261]; 260]);
    //let mut grid = vec![vec![32u8; 110000]; 110000];
    observations.iter().for_each(|v| {
        //println!("x:{} y:{}", v.x, v.y);
        grid[(v.x - offsets.0) as usize][(v.y - offsets.1) as usize] = b'#'
    });

    for y in 0..=max_y - offsets.1 {
        for x in 0..=max_x - offsets.0 {
            print!("{}", grid[x as usize][y as usize] as char);
        }
        print!("\n");
    }
}

fn main() -> io::Result<()> {
    //let mut file = File::open("test_input.txt")?;
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let collection: Vec<&str> = contents.split('\n').collect();
    let collection = &collection[..collection.len() - 1];

    let re = Regex::new(r"position=<(\s*-*\d*),(\s*-*\d*)> velocity=<(\s*-*\d*),(\s*-*\d*)>")
        .expect("Regex creation failed");

    let observations = collection.iter().fold(Vec::new(), |mut v, observation| {
        let capture = re.captures(observation).expect("Failed to capture fields");

        v.push(Observation::new(
            capture.get(1).map_or(0, |m| {
                m.as_str()
                    .trim()
                    .parse()
                    .expect("Failed to parse capture x")
            }),
            capture.get(2).map_or(0, |m| {
                m.as_str()
                    .trim()
                    .parse()
                    .expect("Failed to parse capture y")
            }),
            capture.get(3).map_or(0, |m| {
                m.as_str()
                    .trim()
                    .parse()
                    .expect("Failed to parse capture dx")
            }),
            capture.get(4).map_or(0, |m| {
                m.as_str()
                    .trim()
                    .parse()
                    .expect("Failed to parse capture dy")
            }),
        ));
        v
    });

    for time in 10680..10682 {
        println!("Time {}", time);
        let future = predict(&observations, time);

        let mins = future
            .iter()
            .fold((0i32, 0i32), |(mut min_x, mut min_y), v| {
                if v.x < min_x {
                    min_x = v.x;
                }

                if v.y < min_y {
                    min_y = v.y;
                }

                (min_x, min_y)
            });

        print_grid(&future, mins);
    }

    Ok(())
}
