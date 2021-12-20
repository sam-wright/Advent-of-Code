use std::collections::{HashMap, VecDeque};
use std::f32::consts::PI;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Scanner = Vec<(i32, i32, i32)>;

pub fn read_input(filename: &str) -> Vec<Scanner> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut nodes = Vec::new();
    let mut scanner = Scanner::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("--- scanner") {
            scanner.clear();
            continue;
        } else if line.is_empty() {
            nodes.push(scanner.clone());
            continue;
        }

        let mut entries = line.split(',');
        scanner.push((
            entries.next().unwrap().parse().unwrap(),
            entries.next().unwrap().parse().unwrap(),
            entries.next().unwrap().parse().unwrap(),
        ));
    }

    nodes.push(scanner.clone());

    nodes
}

pub fn rot_x(input: &Scanner, rot: f32) -> Scanner {
    let rot = rot * PI / 180.;
    input
        .iter()
        .map(|&(x, y, z)| {
            (
                x,
                (y * rot.cos() as i32) - (z * rot.sin() as i32),
                (y * rot.sin() as i32) + (z * rot.cos() as i32),
            )
        })
        .collect()
}

pub fn rot_y(input: &Scanner, rot: f32) -> Scanner {
    let rot = rot * PI / 180.;
    input
        .iter()
        .map(|&(x, y, z)| {
            (
                (x * rot.cos() as i32) + (z * rot.sin() as i32),
                y,
                (-x * rot.sin() as i32) + (z * rot.cos() as i32),
            )
        })
        .collect()
}

pub fn rot_z(input: &Scanner, rot: f32) -> Scanner {
    let rot = rot * PI / 180.;
    input
        .iter()
        .map(|&(x, y, z)| {
            (
                (x * rot.cos() as i32) - (y * rot.sin() as i32),
                (x * rot.sin() as i32) + (y * rot.cos() as i32),
                z,
            )
        })
        .collect()
}

pub fn trans_xyz(input: &Scanner, dx: i32, dy: i32, dz: i32) -> Scanner {
    input
        .iter()
        .map(|&(x, y, z)| (x + dx, y + dy, z + dz))
        .collect()
}
pub fn find_translations(
    map: &HashMap<(i32, i32, i32), i32>,
    scanner: &[(i32, i32, i32)],
) -> (i32, (i32, i32, i32)) {
    let mut offsets = HashMap::new();

    for beacon in scanner {
        for point in map.keys() {
            let dx = beacon.0 - point.0;
            let dy = beacon.1 - point.1;
            let dz = beacon.2 - point.2;

            // if dx.abs() >= 2000 || dy.abs() >= 2000 || dz.abs() >= 2000 {
            //     continue;
            // }

            let e = offsets.entry((-dx, -dy, -dz)).or_insert(0);
            *e += 1;
        }
    }

    let mut max = 0;
    let mut offset = (0, 0, 0);
    for (k, v) in offsets {
        if v > max {
            max = v;
            offset = k;
        }
    }

    // dbg!((max, offset));
    (max, offset)
}

pub fn construct_map(
    scanners: &[Scanner],
) -> (
    HashMap<(i32, i32, i32), i32 /* observed*/>,
    Vec<(i32, i32, i32)>,
) {
    let mut scanners: VecDeque<&Scanner> = scanners.iter().collect();
    let mut beacon_map = HashMap::new();

    // Add the first scanner to map verbatim (inherriting its coordinate system)
    scanners.pop_front().unwrap().iter().for_each(|&x| {
        beacon_map.insert(x, 1);
    });

    let mut offsets = vec![(0, 0, 0)];

    'next_scanner: while !scanners.is_empty() {
        let scanner = scanners.pop_front().unwrap();
        let rots = vec![0., 90., 180., 270.];

        for rx in &rots {
            for ry in &rots {
                for rz in &rots {
                    let rotated_scanner = rot_z(&rot_y(&rot_x(scanner, *rx), *ry), *rz);

                    let (matches, (dx, dy, dz)) = find_translations(&beacon_map, &rotated_scanner);

                    if matches >= 12 {
                        let translated_scanner = trans_xyz(&rotated_scanner, dx, dy, dz);
                        offsets.push((dx, dy, dz));

                        for rs in &translated_scanner {
                            let e = beacon_map.entry(*rs).or_insert(0);
                            *e += 1;
                        }

                        continue 'next_scanner;
                    }
                }
            }
        }

        // Didn't find enough connections to the map, try again later
        scanners.push_back(scanner);
    }

    (beacon_map, offsets)
}

pub fn max_offset(offsets: &[(i32, i32, i32)]) -> i32 {
    let a = offsets.clone();
    let b = offsets;

    let mut max_distance = 0;

    for aa in a {
        for bb in b {
            let dist = (aa.0 - bb.0).abs() + (aa.1 - bb.1).abs() + (aa.2 - bb.2).abs();
            max_distance = max_distance.max(dist);
        }
    }
    max_distance
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn rot_test() {
        assert_eq!(rot_y(&vec![(1, 0, 0)], 90.0), [(0, 0, -1),]);
        assert_eq!(rot_z(&vec![(1, 0, 0)], 90.0), [(0, 1, 0),]);

        assert_eq!(rot_x(&vec![(0, 1, 0)], 90.0), [(0, 0, 1),]);
        assert_eq!(rot_z(&vec![(0, 1, 0)], 90.0), [(-1, 0, 0),]);

        assert_eq!(rot_x(&vec![(0, 0, 1)], 90.0), [(0, -1, 0),]);
        assert_eq!(rot_y(&vec![(0, 0, 1)], 90.0), [(1, 0, 0),]);

        assert_eq!(rot_x(&vec![(1, 0, 0)], 90.0), [(1, 0, 0),]);
        assert_eq!(rot_y(&vec![(0, 1, 0)], 90.0), [(0, 1, 0),]);
        assert_eq!(rot_z(&vec![(0, 0, 1)], 90.0), [(0, 0, 1),]);
    }

    #[test]
    fn example1() {
        let scanners = read_input("example.txt");
        let (map, _) = construct_map(&scanners);
        assert_eq!(map.len(), 79);
    }

    #[test]
    fn part1() {
        let scanners = read_input("input.txt");
        let (map, _) = construct_map(&scanners);
        assert_eq!(map.len(), 449);
    }

    #[test]
    fn example2() {
        let scanners = read_input("example.txt");
        let (_, offsets) = construct_map(&scanners);

        assert_eq!(max_offset(&offsets), 3621);
    }

    #[test]
    fn part2() {
        let scanners = read_input("input.txt");
        let (_, offsets) = construct_map(&scanners);

        assert_eq!(max_offset(&offsets), 13128);
    }
}
