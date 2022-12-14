type Type = dyn Fn(u64, Vec<Vec<u64>>) -> Vec<Vec<u64>>;

pub struct Monkey {
    op: Box<dyn Fn(&u64) -> u64>,
    test: Box<Type>,
}

pub fn eval(items: Vec<Vec<u64>>, monkeys: Vec<Monkey>) -> u64 {
    let mut items = items;
    let mut activity = vec![0; items.len()];

    for _round in 0..20 {
        for (idx, monkey) in monkeys.iter().enumerate() {
            while !items[idx].is_empty() {
                let item = &items[idx].pop().unwrap();
                let v = (monkey.op)(item);
                items = (monkey.test)((v as f32 / 3.0).floor() as u64, items);

                activity[idx] += 1;
            }
        }
    }

    activity.sort_unstable();
    activity.reverse();
    activity[0] * activity[1]
}

pub fn ridiculous_eval(items: Vec<Vec<u64>>, monkeys: Vec<Monkey>, gcf: u64) -> u64 {
    let mut items = items;
    let mut activity = vec![0; items.len()];

    for _round in 0..10_000 {
        for (idx, monkey) in monkeys.iter().enumerate() {
            while !items[idx].is_empty() {
                let item = &items[idx].pop().unwrap();
                let v = (monkey.op)(item);
                items = (monkey.test)(v % gcf, items);

                activity[idx] += 1;
            }
        }
    }

    activity.sort_unstable();
    activity.reverse();
    activity[0] * activity[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_test_monkeys() -> Vec<Monkey> {
        vec![
            Monkey {
                op: Box::new(|old| old * 19),
                test: Box::new(|v, mut items| {
                    if v % 23 == 0 {
                        items[2].insert(0, v)
                    } else {
                        items[3].insert(0, v)
                    }
                    items
                }),
            },
            Monkey {
                op: Box::new(|old| old + 6),
                test: Box::new(|v, mut items| {
                    if v % 19 == 0 {
                        items[2].insert(0, v)
                    } else {
                        items[0].insert(0, v)
                    }
                    items
                }),
            },
            Monkey {
                op: Box::new(|old| old * old),
                test: Box::new(|v, mut items| {
                    if v % 13 == 0 {
                        items[1].insert(0, v)
                    } else {
                        items[3].insert(0, v)
                    }
                    items
                }),
            },
            Monkey {
                op: Box::new(|old| old + 3),
                test: Box::new(|v, mut items| {
                    if v % 17 == 0 {
                        items[0].insert(0, v)
                    } else {
                        items[1].insert(0, v)
                    }
                    items
                }),
            },
        ]
    }

    fn make_real_monkeys() -> Vec<Monkey> {
        vec![
            Monkey {
                op: Box::new(|old| old * 7),
                test: Box::new(|v, mut items| {
                    if v % 17 == 0 {
                        items[5].insert(0, v)
                    } else {
                        items[3].insert(0, v)
                    }
                    items
                }),
            },
            Monkey {
                op: Box::new(|old| old + 8),
                test: Box::new(|v, mut items| {
                    if v % 2 == 0 {
                        items[7].insert(0, v)
                    } else {
                        items[6].insert(0, v)
                    }
                    items
                }),
            },
            Monkey {
                op: Box::new(|old| old * 13),
                test: Box::new(|v, mut items| {
                    if v % 5 == 0 {
                        items[1].insert(0, v)
                    } else {
                        items[6].insert(0, v)
                    }
                    items
                }),
            },
            Monkey {
                op: Box::new(|old| old + 7),
                test: Box::new(|v, mut items| {
                    if v % 3 == 0 {
                        items[5].insert(0, v)
                    } else {
                        items[2].insert(0, v)
                    }
                    items
                }),
            },
            Monkey {
                op: Box::new(|old| old + 2),
                test: Box::new(|v, mut items| {
                    if v % 7 == 0 {
                        items[0].insert(0, v)
                    } else {
                        items[3].insert(0, v)
                    }
                    items
                }),
            },
            Monkey {
                op: Box::new(|old| old + 1),
                test: Box::new(|v, mut items| {
                    if v % 13 == 0 {
                        items[2].insert(0, v)
                    } else {
                        items[1].insert(0, v)
                    }
                    items
                }),
            },
            Monkey {
                op: Box::new(|old| old + 4),
                test: Box::new(|v, mut items| {
                    if v % 19 == 0 {
                        items[7].insert(0, v)
                    } else {
                        items[4].insert(0, v)
                    }
                    items
                }),
            },
            Monkey {
                op: Box::new(|old| old * old),
                test: Box::new(|v, mut items| {
                    if v % 11 == 0 {
                        items[0].insert(0, v)
                    } else {
                        items[4].insert(0, v)
                    }
                    items
                }),
            },
        ]
    }

    #[test]
    fn example1() {
        let items = vec![
            vec![98, 79],
            vec![74, 75, 65, 54],
            vec![97, 60, 79],
            vec![74],
        ];

        let monkeys = make_test_monkeys();
        assert_eq!(eval(items, monkeys), 10605);
    }

    #[test]
    fn part1() {
        let items = vec![
            vec![74, 63, 97, 61, 54],
            vec![87, 52, 83, 99, 64, 97, 70, 61],
            vec![65, 80, 67, 60],
            vec![56, 82, 69, 76, 70, 61],
            vec![98, 79],
            vec![55, 79, 72],
            vec![63],
            vec![81, 86, 80, 63, 93, 51, 72],
        ];

        let monkeys = make_real_monkeys();
        assert_eq!(eval(items, monkeys), 50172);
    }

    #[test]
    fn example2() {
        let items = vec![
            vec![98, 79],
            vec![74, 75, 65, 54],
            vec![97, 60, 79],
            vec![74],
        ];

        let monkeys = make_test_monkeys();
        assert_eq!(
            ridiculous_eval(items, monkeys, 19 * 23 * 13 * 17),
            2713310158
        );
    }

    #[test]
    fn part2() {
        let items = vec![
            vec![74, 63, 97, 61, 54],
            vec![87, 52, 83, 99, 64, 97, 70, 61],
            vec![65, 80, 67, 60],
            vec![56, 82, 69, 76, 70, 61],
            vec![98, 79],
            vec![55, 79, 72],
            vec![63],
            vec![81, 86, 80, 63, 93, 51, 72],
        ];

        let monkeys = make_real_monkeys();
        assert_eq!(
            ridiculous_eval(items, monkeys, 17 * 2 * 5 * 3 * 7 * 13 * 19 * 11),
            11614682178
        );
    }
}
