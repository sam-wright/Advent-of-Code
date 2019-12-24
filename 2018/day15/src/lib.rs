use std::fs::File;
use std::io::{self, BufRead};

enum Action {
    Move,
    Attack,
}

struct Unit{
    unit_type:Type,
    x:u32,
    y:u32,
    hp:u32,
}
enum Type{
    Goblin,
    Elf,
}

type Map = Vec<Vec<u8>>;
pub struct Game {
    map: Map,
    units: Vec<Unit>,
}
pub fn read_map(filename: &str) -> Game {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => panic!("Bad filename!"),
    };

    let lines = io::BufReader::new(file).lines();

    let mut game = Game { map: Vec::new() };
    for line in lines {
        game.map
            .push(line.expect("Unable to read line").into_bytes());
    }
    game
}

impl Game {
    pub fn print_map(&self) {
        for line in &self.map {
            for c in line {
                print!("{}", *c as char);
            }
            println!();
        }
    }

}

#[test]
pub fn test_input_1() {
    let game = read_map("testinput1.txt");
    game.print_map();
}


#[test]
fn test_input_2() {
    let game = read_map("testinput2.txt");
    game.print_map();
}
