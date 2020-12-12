use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub enum Action {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

#[derive(Debug)]
pub enum Direction {
    N,
    S,
    E,
    W,
}
impl Direction {
    fn increment(&self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::S => Direction::W,
            Direction::E => Direction::S,
            Direction::W => Direction::N,
        }
    }
    fn decrement(&self) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
            Direction::W => Direction::S,
        }
    }
}

#[derive(Debug)]
pub struct ShipActor {
    heading: Direction,
    x_pos: i32,
    y_pos: i32,
}

impl ShipActor {
    pub fn new() -> Self {
        ShipActor {
            heading: Direction::E,
            y_pos: 0,
            x_pos: 0,
        }
    }

    pub fn distance(&self) -> i32 {
        self.x_pos.abs() + self.y_pos.abs()
    }

    pub fn apply_action(&mut self, action: &Action) {
        match action {
            Action::N(v) => self.y_pos += v,
            Action::S(v) => self.y_pos -= v,
            Action::E(v) => self.x_pos -= v,
            Action::W(v) => self.x_pos += v,
            Action::L(v) => {
                for _ in 0..(v / 90) {
                    self.heading = self.heading.increment();
                }
            }
            Action::R(v) => {
                for _ in 0..(v / 90) {
                    self.heading = self.heading.decrement();
                }
            }
            Action::F(v) => match self.heading {
                Direction::N => self.y_pos -= v,
                Direction::S => self.y_pos += v,
                Direction::E => self.x_pos -= v,
                Direction::W => self.x_pos += v,
            },
        }
    }
}
#[derive(Debug)]
pub struct WaypointActor {
    ship: ShipActor,
    pub x_pos: i32,
    pub y_pos: i32,
}

impl WaypointActor {
    pub fn new() -> Self {
        WaypointActor {
            ship: ShipActor::new(),
            y_pos: 1,
            x_pos: -10,
        }
    }
    pub fn distance(&self) -> i32 {
        self.ship.distance()
    }

    pub fn apply_action(&mut self, action: &Action) {
        match action {
            Action::N(v) => self.y_pos += v,
            Action::S(v) => self.y_pos -= v,
            Action::E(v) => self.x_pos -= v,
            Action::W(v) => self.x_pos += v,
            Action::R(v) => {
                for _ in 0..(v / 90) {
                    let (px, py) = (self.x_pos, self.y_pos);

                    let (x, y) = match (px.is_positive(), py.is_positive()) {
                        (true, true) => (-py, px),
                        (false, true) => (-py, px),
                        (true, false) => (-py, px),
                        (false, false) => (-py, px),
                    };
                    self.x_pos = x;
                    self.y_pos = y;
                }
            }
            Action::L(v) => {
                for _ in 0..(v / 90) {
                    let (px, py) = (self.x_pos, self.y_pos);

                    let (x, y) = match (px.is_positive(), py.is_positive()) {
                        (true, true) => (py, -px),
                        (false, true) => (py, -px),
                        (true, false) => (py, -px),
                        (false, false) => (py, -px),
                    };
                    self.x_pos = x;
                    self.y_pos = y;
                }
            }
            Action::F(v) => {
                for _ in 0..*v {
                    self.ship.x_pos += self.x_pos;
                    self.ship.y_pos += self.y_pos;
                }
            }
        }
    }
}

pub fn read_input(filename: &str) -> Vec<Action> {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let collection: Vec<&str> = contents.split("\n").collect();

    collection
        .iter()
        .map(|x| {
            let mut c = x.chars();
            let t = c.next().unwrap();
            let v = c.as_str();
            match t {
                'N' => Action::N(v.parse().unwrap()),
                'S' => Action::S(v.parse().unwrap()),
                'E' => Action::E(v.parse().unwrap()),
                'W' => Action::W(v.parse().unwrap()),
                'L' => Action::L(v.parse().unwrap()),
                'R' => Action::R(v.parse().unwrap()),
                'F' => Action::F(v.parse().unwrap()),
                _ => panic!(""),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let instructions = read_input("example1.txt");

        let mut actor = ShipActor::new();
        for inst in &instructions {
            actor.apply_action(inst);
        }
        assert_eq!(actor.distance(), 25);
    }

    #[test]
    fn part_1_solution() {
        let instructions = read_input("input.txt");

        let mut actor = ShipActor::new();
        for inst in &instructions {
            actor.apply_action(inst);
        }
        assert_eq!(actor.distance(), 582);
    }

    #[test]
    fn part_2_example_a() {
        let instructions = read_input("example1.txt");

        let mut actor = WaypointActor::new();
        for inst in &instructions {
            actor.apply_action(inst);
        }

        assert_eq!(actor.distance(), 286);
    }
    #[test]
    fn part_2_example_b() {
        let instructions = read_input("example1_alt.txt");

        let mut actor = WaypointActor::new();
        for inst in &instructions {
            actor.apply_action(inst);
        }

        assert_eq!(actor.distance(), 286);
    }
    #[test]
    fn part_2_solution() {
        let instructions = read_input("input.txt");

        let mut actor = WaypointActor::new();
        for inst in &instructions {
            actor.apply_action(inst);
        }

        assert_eq!(actor.distance(), 52069);
    }
}
