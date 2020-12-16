use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

pub fn read_input(filename: &str) -> Vec<Vec<i32>> {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let collection: Vec<Vec<i32>> = contents
        .split("\n")
        .map(|x| x.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();
    collection
}

#[derive(Debug)]
pub struct Constraint {
    min: i32,
    max: i32,
}

impl Constraint {
    pub fn new(min: i32, max: i32) -> Self {
        Constraint { min, max }
    }
}

// the entirety of part-1
pub fn check_constraints(tickets: &Vec<Vec<i32>>, rules: &Vec<Vec<Constraint>>) -> Vec<i32> {
    let mut bads = Vec::new();
    for ticket in tickets {
        'outer: for field in ticket.iter() {
            // check against each rule

            for rule in rules {
                for rule_item in rule {
                    if field >= &rule_item.min && field <= &rule_item.max {
                        // println!("item: {} meets {:?}", field, rule_item);
                        continue 'outer;
                    }
                }
            }
            bads.push(*field);
        }
    }
    bads
}

// could be replaced by single filter
pub fn get_valid_tickets(tickets: &Vec<Vec<i32>>, blacklist: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut valid_tickets = Vec::new();
    'outer: for ticket in tickets {
        for entry in ticket {
            if blacklist.contains(entry) {
                continue 'outer;
            }
        }
        valid_tickets.push(ticket.clone());
    }

    valid_tickets
}

pub fn get_constraint_columns(
    tickets: &Vec<Vec<i32>>,
    rules: &Vec<Vec<Constraint>>,
    blacklist: &Vec<i32>,
) -> Vec<HashSet<usize>> {
    let valid_tickets = get_valid_tickets(&tickets, &blacklist);
    let all_constraints: HashSet<usize> = (0..rules.len()).collect();
    let mut valid_constraints = Vec::new();

    for ticket_col in 0..rules.len() {
        // for each colum apply each rule
        let mut invalid_rules = Vec::new();

        for ticket in &valid_tickets {
            let field = ticket[ticket_col];
            for (rule_id, rule) in rules.iter().enumerate() {
                if (field < rule[0].min || field > rule[0].max)
                    && (field < rule[1].min || field > rule[1].max)
                {
                    // println!(
                    //     "Constraint {} is invalid for ticket_col {}",
                    //     rule_id, ticket_col
                    // );
                    invalid_rules.push(rule_id);

                    break;
                    // break 'outer;
                }
            }
        }
        let x: HashSet<usize> = invalid_rules.iter().map(|x| *x).collect();
        valid_constraints.push(all_constraints.difference(&x).map(|x| *x).collect());
    }

    valid_constraints
}

// Eeeew, you know you're off to a ropey solution when you start solving using Rng...
// But it works
pub fn solve(constraints: Vec<HashSet<usize>>) {
    // reduce it as much as is statically determined
    let (mut solved, constraints) = reduce(constraints);

    // begin guessing
    while !solved {
        let mut rng = rand::thread_rng();

        let mut new_guess = Vec::new();

        // pick a random index from the possibly valid solutions and check for success
        for i in 0..constraints.len() {
            let constraint = constraints[i].clone();
            if constraint.len() > 1 {
                let guessed_idx = rng.gen_range(0, constraints[i].len());
                let val = constraints[i].iter().nth(guessed_idx).unwrap();
                new_guess.push([val.clone()].iter().map(|x| *x).collect());
            } else {
                new_guess.push(constraint);
            }
        }
        let (success, ans) = reduce(new_guess);
        solved = success;
        if success {
            dbg!(ans);
        }
    }
    println!("solved!");
}

pub fn reduce(mut constraints: Vec<HashSet<usize>>) -> (bool, Vec<HashSet<usize>>) {
    let mut solved = false;
    let mut counter = 0;
    while !solved && counter < 100 {
        solved = true;

        for i in 0..constraints.len() {
            if constraints[i].len() == 1 {
                let value = constraints[i].iter().next().unwrap().clone();
                // solved constraint, remove as possible solution for other constraints
                for j in 0..constraints.len() {
                    if j == i {
                        continue;
                    }
                    constraints[j] = constraints[j]
                        .iter()
                        .cloned()
                        .filter(|&x| x != value)
                        .collect();
                }
            } else {
                solved = false
            }
            counter += 1;
        }
    }
    (solved, constraints)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let constraints = vec![
            vec![Constraint::new(1, 3), Constraint::new(5, 7)],
            vec![Constraint::new(6, 11), Constraint::new(33, 44)],
            vec![Constraint::new(13, 40), Constraint::new(45, 50)],
        ];
        let nearby_tickets = vec![
            vec![7, 3, 47],
            vec![40, 4, 50],
            vec![55, 2, 20],
            vec![38, 6, 12],
        ];

        let sum = check_constraints(&nearby_tickets, &constraints);
        assert_eq!(sum.iter().sum::<i32>(), 71);
    }

    #[test]
    fn part_1_solution() {
        let constraints = vec![
            vec![Constraint::new(34, 724), Constraint::new(735, 974)], // departure location
            vec![Constraint::new(40, 521), Constraint::new(534, 950)], // departure station
            vec![Constraint::new(40, 329), Constraint::new(353, 973)], // departure platform
            vec![Constraint::new(37, 258), Constraint::new(268, 964)], // departure track
            vec![Constraint::new(32, 650), Constraint::new(665, 964)], // departure date
            vec![Constraint::new(39, 373), Constraint::new(398, 950)], // departure time
            vec![Constraint::new(42, 431), Constraint::new(447, 952)], // arrival location
            vec![Constraint::new(36, 536), Constraint::new(552, 972)], // arrival station
            vec![Constraint::new(45, 666), Constraint::new(678, 952)], // arrival platform
            vec![Constraint::new(49, 836), Constraint::new(852, 952)], // arrival track
            vec![Constraint::new(35, 600), Constraint::new(623, 953)], // class:
            vec![Constraint::new(50, 920), Constraint::new(929, 950)], // duration:
            vec![Constraint::new(35, 853), Constraint::new(870, 973)], // price:
            vec![Constraint::new(34, 309), Constraint::new(318, 965)], // route:
            vec![Constraint::new(42, 267), Constraint::new(292, 962)], // row:
            vec![Constraint::new(46, 632), Constraint::new(642, 954)], // seat:
            vec![Constraint::new(47, 746), Constraint::new(754, 960)], // train:
            vec![Constraint::new(32, 406), Constraint::new(423, 963)], // type:
            vec![Constraint::new(37, 797), Constraint::new(810, 973)], // wagon:
            vec![Constraint::new(35, 766), Constraint::new(784, 952)], // zone:
        ];
        let nearby_tickets = read_input("input_nearby_tickets.txt");

        let sum = check_constraints(&nearby_tickets, &constraints);
        assert_eq!(sum.iter().sum::<i32>(), 24021);
    }

    #[test]
    fn part_2_example_a() {
        let constraints = vec![
            vec![Constraint::new(1, 3), Constraint::new(5, 7)],
            vec![Constraint::new(6, 11), Constraint::new(33, 44)],
            vec![Constraint::new(13, 40), Constraint::new(45, 50)],
        ];
        let nearby_tickets = vec![
            vec![7, 3, 47],
            vec![40, 4, 50],
            vec![55, 2, 20],
            vec![38, 6, 12],
        ];

        let bads = check_constraints(&nearby_tickets, &constraints);

        let x = get_valid_tickets(&nearby_tickets, &bads);
        assert_eq!(x, &[&[7, 3, 47]]);

        let constraints = get_constraint_columns(&nearby_tickets, &constraints, &bads);
        solve(constraints);
    }

    #[test]
    fn part_2_example_b() {
        let constraints = vec![
            vec![Constraint::new(0, 1), Constraint::new(4, 19)],
            vec![Constraint::new(0, 5), Constraint::new(8, 19)],
            vec![Constraint::new(0, 13), Constraint::new(16, 19)],
        ];
        let nearby_tickets = vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9]];

        let bads = check_constraints(&nearby_tickets, &constraints);

        let x = get_valid_tickets(&nearby_tickets, &bads);
        assert_eq!(x, [[3, 9, 18], [15, 1, 5], [5, 14, 9]]);

        let constraints = get_constraint_columns(&nearby_tickets, &constraints, &bads);
        solve(constraints);
    }

    #[test]
    fn part_2_solution() {
        let constraints = vec![
            vec![Constraint::new(34, 724), Constraint::new(735, 974)], // departure location
            vec![Constraint::new(40, 521), Constraint::new(534, 950)], // departure station
            vec![Constraint::new(40, 329), Constraint::new(353, 973)], // departure platform
            vec![Constraint::new(37, 258), Constraint::new(268, 964)], // departure track
            vec![Constraint::new(32, 650), Constraint::new(665, 964)], // departure date
            vec![Constraint::new(39, 373), Constraint::new(398, 950)], // departure time
            vec![Constraint::new(42, 431), Constraint::new(447, 952)], // arrival location
            vec![Constraint::new(36, 536), Constraint::new(552, 972)], // arrival station
            vec![Constraint::new(45, 666), Constraint::new(678, 952)], // arrival platform
            vec![Constraint::new(49, 836), Constraint::new(852, 952)], // arrival track
            vec![Constraint::new(35, 600), Constraint::new(623, 953)], // class:
            vec![Constraint::new(50, 920), Constraint::new(929, 950)], // duration:
            vec![Constraint::new(35, 853), Constraint::new(870, 973)], // price:
            vec![Constraint::new(34, 309), Constraint::new(318, 965)], // route:
            vec![Constraint::new(42, 267), Constraint::new(292, 962)], // row:
            vec![Constraint::new(46, 632), Constraint::new(642, 954)], // seat:
            vec![Constraint::new(47, 746), Constraint::new(754, 960)], // train:
            vec![Constraint::new(32, 406), Constraint::new(423, 963)], // type:
            vec![Constraint::new(37, 797), Constraint::new(810, 973)], // wagon:
            vec![Constraint::new(35, 766), Constraint::new(784, 952)], // zone:
        ];
        let nearby_tickets = read_input("input_nearby_tickets.txt");

        let bads = check_constraints(&nearby_tickets, &constraints);
        let constraints = get_constraint_columns(&nearby_tickets, &constraints, &bads);
        solve(constraints);

        let tickets: Vec<usize> = vec![
            113, 53, 97, 59, 139, 73, 89, 109, 67, 71, 79, 127, 149, 107, 137, 83, 131, 101, 61,
            103,
        ];
        let ans = tickets[20 - 1]
            * tickets[17 - 1]
            * tickets[11 - 1]
            * tickets[14 - 1]
            * tickets[12 - 1]
            * tickets[7 - 1];
        assert_eq!(ans, 1289178686687);
    }
}
