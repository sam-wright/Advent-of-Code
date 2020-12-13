use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Bus {
    id: i32,
    departure_times: Vec<i32>,
}

impl Bus {
    pub fn new(id: i32, time: i32) -> Self {
        let departure_times: Vec<i32> = (0..time + id).step_by(id as usize).collect();
        Self {
            id,
            departure_times,
        }
    }

    pub fn nearest_departure(&self, time: i32) -> i32 {
        let mut soonest = 9999999;

        for i in 0..2 {
            let delta = self.departure_times[self.departure_times.len() - 1 - i] - time;
            if delta < soonest && delta > 0 {
                soonest = delta;
            }
        }
        soonest
    }
}

#[derive(Debug)]
pub struct Schedule {
    time: i32,
    busses: Vec<Bus>,
}

impl Schedule {
    pub fn new(input: &[String]) -> Self {
        let time = input[0].parse().unwrap();

        let busses: Vec<Bus> = input[1]
            .split(",")
            .filter_map(|x| x.parse::<i32>().ok())
            .map(|x| Bus::new(x, time))
            .collect();

        Schedule { time, busses }
    }

    pub fn nearest_departure(&self) -> (i32, i32) {
        let mut soonest_bus = -1;
        let mut soonest_time = 99999;

        for b in &self.busses {
            let delta = b.nearest_departure(self.time);
            if delta < soonest_time {
                soonest_time = delta;
                soonest_bus = b.id;
            }
        }

        (soonest_bus, soonest_time)
    }
}

pub fn read_input(filename: &str) -> Vec<String> {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let collection: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    collection
}

//
// Ok, the part-1 implementation seems useless for solving part-2
//

#[derive(Debug)]
pub struct Route {
    freq: usize,
}

#[derive(Debug)]
pub struct CoordinatedSchedule {
    routes: Vec<Route>,
}

impl CoordinatedSchedule {
    pub fn new(input: &[String]) -> Self {
        let routes: Vec<Route> = input[1]
            .split(",")
            .map(|x| {
                let freq = if x == "x" { 1 } else { x.parse().unwrap() };
                Route { freq }
            })
            .collect();
        CoordinatedSchedule { routes }
    }

    pub fn coordinate(&mut self) -> usize {
        let mut idx = self.routes[0].freq;
        let mut complete = false;

        while !complete {
            complete = true;
            let mut stride = 1;

            for (i, route) in self.routes.iter().enumerate() {
                if (idx + i) % route.freq == 0 {
                    stride *= route.freq;
                } else {
                    complete = false;
                    idx += stride;
                    break;
                }
            }
        }

        idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = read_input("example1.txt");
        let schedule = Schedule::new(&input);

        let r = schedule.nearest_departure();

        assert_eq!(r.0 * r.1, 295);
    }

    #[test]
    fn part_1_solution() {
        let input = read_input("input.txt");

        let schedule = Schedule::new(&input);

        let r = schedule.nearest_departure();

        assert_eq!(r.0 * r.1, 174);
    }

    #[test]
    fn part_2_example() {
        let mut schedule = CoordinatedSchedule::new(&read_input("example2a.txt"));
        assert_eq!(schedule.coordinate(), 3417);

        let mut schedule = CoordinatedSchedule::new(&read_input("example2b.txt"));
        assert_eq!(schedule.coordinate(), 754018);

        let mut schedule = CoordinatedSchedule::new(&read_input("example2c.txt"));
        assert_eq!(schedule.coordinate(), 779210);

        let mut schedule = CoordinatedSchedule::new(&read_input("example2d.txt"));
        assert_eq!(schedule.coordinate(), 1261476);

        let mut schedule = CoordinatedSchedule::new(&read_input("example2e.txt"));
        assert_eq!(schedule.coordinate(), 1202161486);
    }

    #[test]
    fn part_2_solution() {
        let mut schedule = CoordinatedSchedule::new(&read_input("input.txt"));

        assert_eq!(schedule.coordinate(), 780601154795940);
    }
}
