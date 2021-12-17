use std::cmp::Ordering;

#[derive(Debug)]
pub struct Target {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

#[derive(Debug)]
pub struct Probe {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

pub fn hit_target(p: &Probe, t: &Target) -> bool {
    p.x >= t.xmin && p.x <= t.xmax && p.y >= t.ymin && p.y <= t.ymax
}

pub fn miss_target(p: &Probe, t: &Target) -> bool {
    p.y < t.ymin
}

pub fn shoot(probe: &Probe, target: &Target, max_y: i32) -> Option<i32 /*max_height*/> {
    let max_y = max_y.max(probe.y);

    if hit_target(probe, target) {
        return Some(max_y);
    } else if miss_target(probe, target) {
        return None;
    }

    let p = Probe {
        x: probe.x + probe.dx,
        y: probe.y + probe.dy,
        dy: probe.dy - 1,
        dx: match probe.dx.cmp(&0) {
            Ordering::Less => probe.dx + 1,
            Ordering::Equal => 0,
            Ordering::Greater => probe.dx - 1,
        },
    };
    shoot(&p, target, max_y)
}

pub fn find_best_shot(target: &Target) -> (i32, i32, i32) {
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut max_h = i32::MIN;

    for dx in 0..100 {
        for dy in -100..200 {
            let probe = Probe { x: 0, y: 0, dx, dy };

            if let Some(h) = shoot(&probe, target, 0) {
                if h > max_h {
                    max_h = h;
                    max_x = dx;
                    max_y = dy;
                }
            }
        }
    }

    (max_x, max_y, max_h)
}

pub fn find_shots(target: &Target) -> i32 {
    let mut hits = 0;

    for dx in 0..500 {
        for dy in -500..500 {
            let probe = Probe { x: 0, y: 0, dx, dy };

            if shoot(&probe, target, 0).is_some() {
                hits += 1
            }
        }
    }

    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let target = Target {
            xmin: 20,
            xmax: 30,
            ymin: -10,
            ymax: -5,
        };

        let probe1 = Probe {
            x: 0,
            y: 0,
            dx: 17,
            dy: -4,
        };

        let probe2 = Probe {
            x: 0,
            y: 0,
            dx: 6,
            dy: 9,
        };

        assert_eq!(shoot(&probe1, &target, 0), None);
        assert_eq!(shoot(&probe2, &target, 0), Some(45));

        assert_eq!(find_best_shot(&target), (6, 9, 45));
    }

    #[test]
    fn part1() {
        // target area: x=94..151, y=-156..-103
        let target = Target {
            xmin: 94,
            xmax: 151,
            ymin: -156,
            ymax: -103,
        };

        assert_eq!(find_best_shot(&target), (14, 155, 12090));
    }

    #[test]
    fn example2() {
        let target = Target {
            xmin: 20,
            xmax: 30,
            ymin: -10,
            ymax: -5,
        };
        assert_eq!(find_shots(&target), 112);
    }

    #[test]
    fn part2() {
        let target = Target {
            xmin: 94,
            xmax: 151,
            ymin: -156,
            ymax: -103,
        };
        assert_eq!(find_shots(&target), 5059);
        //4664 low
        //13008 high
    }
}
