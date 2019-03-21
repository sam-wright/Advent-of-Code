use std::cell::RefCell;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Index;
use std::process::exit;

#[derive(Eq, Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Direction) -> bool {
        self == other
    }
}

#[derive(Debug, Eq, Clone)]
struct Cart {
    id: usize,
    x: RefCell<usize>,
    y: RefCell<usize>,
    intersection_count: RefCell<i32>,
    direction: RefCell<Direction>,
    alive: Box<RefCell<bool>>,
}

impl Cart {
    fn new(id: usize, x: usize, y: usize, direction: Direction) -> Cart {
        Cart {
            x: RefCell::new(x),
            y: RefCell::new(y),
            id,
            intersection_count: RefCell::new(0),
            direction: RefCell::new(direction),
            alive: Box::new(RefCell::new(true)),
        }
    }

    fn action(&self) {
        if !*self.alive.borrow() {
            return;
        }

        print!(
            "\tMoving {} from x:{} y:{}\t{:?}\t",
            self.id,
            self.x.borrow(),
            self.y.borrow(),
            self.direction.borrow()
        );
        match *self.direction.borrow() {
            Direction::Up => *self.y.borrow_mut() -= 1,
            Direction::Down => *self.y.borrow_mut() += 1,
            Direction::Left => *self.x.borrow_mut() -= 1,
            Direction::Right => *self.x.borrow_mut() += 1,
        }
        println!("to x:{} y:{}", self.x.borrow(), self.y.borrow());
    }

    fn reorient(&self, mmap: &Map) {
        if !*self.alive.borrow() {
            return;
        }
        let current_direction = self.direction.borrow().clone();

        let track = mmap[*self.y.borrow()][*self.x.borrow()];

        self.direction.replace(match current_direction {
            Direction::Up => match track {
                b'/' => Direction::Right,
                b'\\' => Direction::Left,
                b'+' => {
                    *self.intersection_count.borrow_mut() += 1;
                    match *self.intersection_count.borrow() % 3 {
                        0 => Direction::Right,
                        1 => Direction::Left,
                        2 => Direction::Up,
                        _ => panic!("How did I ever get here?"),
                    }
                }
                b' ' => panic!(format!(
                    "Off the track! how did I get here? x:{} y:{}",
                    *self.x.borrow(),
                    *self.y.borrow()
                )),
                _ => Direction::Up,
            },
            Direction::Down => match track {
                b'/' => Direction::Left,
                b'\\' => Direction::Right,
                b'+' => {
                    *self.intersection_count.borrow_mut() += 1;
                    match *self.intersection_count.borrow() % 3 {
                        0 => Direction::Left,
                        1 => Direction::Right,
                        2 => Direction::Down,
                        _ => panic!("How did I ever get here?"),
                    }
                }
                b' ' => panic!(format!(
                    "Off the track! how did I get here? x:{} y:{}",
                    *self.x.borrow(),
                    *self.y.borrow()
                )),
                _ => Direction::Down,
            },
            Direction::Left => match track {
                b'/' => Direction::Down,
                b'\\' => Direction::Up,
                b'+' => {
                    *self.intersection_count.borrow_mut() += 1;
                    match *self.intersection_count.borrow() % 3 {
                        0 => Direction::Up,
                        1 => Direction::Down,
                        2 => Direction::Left,
                        _ => panic!("How did I ever get here?"),
                    }
                }
                b' ' => panic!(format!(
                    "Off the track! how did I get here? x:{} y:{}",
                    *self.x.borrow(),
                    *self.y.borrow()
                )),
                _ => Direction::Left,
            },
            Direction::Right => match track {
                b'/' => Direction::Up,
                b'\\' => Direction::Down,
                b'+' => {
                    *self.intersection_count.borrow_mut() += 1;
                    match *self.intersection_count.borrow() % 3 {
                        0 => Direction::Down,
                        1 => Direction::Up,
                        2 => Direction::Right,
                        _ => panic!("How did I ever get here?"),
                    }
                }
                b' ' => panic!(format!(
                    "Off the track! how did I get here? x:{} y:{}",
                    *self.x.borrow(),
                    *self.y.borrow()
                )),
                _ => Direction::Right,
            },
        });
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        (*self.y.borrow() * 999 + *self.x.borrow())
            .cmp(&(*other.y.borrow() * 999 + *other.x.borrow()))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        *self.x.borrow() == *other.x.borrow() && *self.y.borrow() == *other.y.borrow()
    }
}

type Map = Vec<Vec<u8>>;
fn read_map(filename: &str) -> Map {
    let file = match File::open(filename) {
        Ok(file) => file,
        _ => panic!("no such file"),
    };
    let lines = io::BufReader::new(file).lines();

    let mut map = Vec::new();
    for line in lines {
        map.push(
            line.expect("Unable to read line")
                .replace("v", "|")
                .replace("^", "|")
                .replace(">", "-")
                .replace("<", "-")
                .into_bytes(),
        );
    }
    map
}

fn print_map(map: &Map) {
    map.iter().for_each(|y| {
        y.iter().for_each(|&x| print!("{}", x as char));
        print!("\n");
    });
}

fn print_map_with_carts(map: &Map, carts: &[Cart]) {
    let mut tmp_map = map.clone();
    for cart in carts {
        if *cart.alive.borrow() {
            tmp_map[*cart.y.borrow()][*cart.x.borrow()] = match *cart.direction.borrow() {
                Direction::Up => b'^',
                Direction::Left => b'<',
                Direction::Right => b'>',
                Direction::Down => b'v',
            };
        } else {
            tmp_map[*cart.y.borrow()][*cart.x.borrow()] = b'O';
        }
    }

    tmp_map.iter().for_each(|y| {
        y.iter().for_each(|&x| print!("{}", x as char));
        println!("");
    });
}
fn read_carts(filename: &str) -> Vec<Cart> {
    let file = match File::open(filename) {
        Ok(file) => file,
        _ => panic!("no such file"),
    };
    let lines = io::BufReader::new(file).lines();

    let mut carts = Vec::with_capacity(30);
    lines.enumerate().for_each(|(y, line)| {
        let line = line.expect("unable to read line").clone();

        if let Some(x) = line.find('v') {
            carts.push(Cart::new(x * 10 + y, x, y, Direction::Down))
        }

        if let Some(x) = line.find('^') {
            carts.push(Cart::new(x * 10 + y, x, y, Direction::Up))
        }

        if let Some(x) = line.find('>') {
            carts.push(Cart::new(x * 10 + y, x, y, Direction::Right))
        }

        if let Some(x) = line.find('<') {
            carts.push(Cart::new(x * 10 + y, x, y, Direction::Left))
        }
    });
    carts
}

fn check_collisions(carts: &[Cart]) -> bool {
    let mut copy = carts.to_owned();

    copy.sort();
    copy.dedup();

    carts.len() != copy.len()
}

fn get_cart_order(carts: &[Cart]) -> Vec<usize> {
    assert!(carts.len() % 2 == 1);

    let mut order = Vec::with_capacity(carts.len());

    let mut sorted_carts = carts.to_owned();
    sorted_carts.sort();

    sorted_carts
        .iter()
        .filter(|&c| *c.alive.borrow())
        .for_each(|cart| {
            order.push(
                carts
                    .iter()
                    .position(|c| c == cart && *cart.alive.borrow() && *c.alive.borrow())
                    .unwrap(),
            )
        });
    for i in &order {
        if !(*carts[*i].alive.borrow()) {
            println!("Invalid");
            exit(-1);
        }
    }

    order
}

fn remove_crashed(carts: &[Cart]) -> usize {
    for cart in carts.iter() {
        for cart_ in carts.iter() {
            if cart == cart_ && cart.id != cart_.id && *cart.alive.borrow() && *cart_.alive.borrow()
            {
                cart.alive.replace(false);
                cart_.alive.replace(false);
            }
        }
    }

    carts.iter().fold(
        0usize,
        |sum, cart| {
            if *cart.alive.borrow() {
                sum + 1
            } else {
                sum
            }
        },
    )
}

fn main() {
    //let filename = "test_input1.txt";
    //let filename = "test_input2.txt";
    //let filename = "test_input3.txt";
    //let filename = "test_input4.txt";
    //let filename = "test2_input1.txt";

    let filename = "input.txt";
    //let filename = "test_input5.txt";

    let map = read_map(&filename);

    let carts: Vec<Cart> = read_carts(filename);
    for tick in 0..1_000_000 {
        println!("iteration {}", &tick);
        let order = get_cart_order(&carts);
        println!("Cart Order: ");
        for o in &order {
            println!("{} ", o);
        }
        println!("\n");
        for index in &order {
            println!(
                "Cart:{} ({}) -> {:?}",
                &index,
                carts.index(*index).id,
                carts.index(*index).alive
            );

            carts.index(*index).action();
            // Part-A
            //if check_collisions(&carts) {
            //    println!("Collision Detected!!");
            //print_map_with_carts(&map, &carts);
            //exit(1);
            // Part-B
            let _ = remove_crashed(&carts);
            //}
        }

        carts.iter().for_each(|cart| cart.reorient(&map));

        if remove_crashed(&carts) <= 1 {
            print_map_with_carts(&map, &carts);

            dbg!(&carts);
            exit(1);
        }
    }
}
