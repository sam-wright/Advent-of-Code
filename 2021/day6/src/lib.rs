use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

pub fn read_input(filename: &str) -> Vec<usize> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut data = String::new();

    reader.read_to_string(&mut data).unwrap();
    data.split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

pub fn next_day(state: Vec<usize>) -> Vec<usize> {
    let mut new_state = Vec::new();

    for timer in state {
        if timer > 0 {
            new_state.push(timer - 1);
        } else if timer == 0 {
            new_state.push(6);
            new_state.push(8);
        }
    }

    new_state
}

pub fn next_day2(mut state: Vec<usize>) -> Vec<usize> {
    let mut new_state = Vec::new();

    for timer in &mut state {
        if *timer > 0 {
            *timer -= 1;
        } else if *timer == 0 {
            *timer = 6;
            new_state.push(8);
        }
    }

    state.append(&mut new_state);
    state
}

pub fn next_day3(
    state: HashMap<usize /*timer*/, usize /*count*/>,
) -> HashMap<usize /*timer*/, usize /*count*/> {
    let mut new_state = HashMap::new();

    for (timer, count) in state {
        if timer > 0 {
            let e = new_state.entry(timer - 1).or_insert(0);
            *e += count;
        } else if timer == 0 {
            let e = new_state.entry(6).or_insert(0);
            *e += count;
            let e = new_state.entry(8).or_insert(0);
            *e += count;
        }
    }

    new_state
}

pub fn vec2hashy(state: Vec<usize>) -> HashMap<usize /*timer*/, usize /*count*/> {
    let mut new_state = HashMap::new();

    for s in state {
        let e = new_state.entry(s).or_insert(0);
        *e += 1;
    }

    new_state
}

pub fn count_size(state: &HashMap<usize /*timer*/, usize /*count*/>) -> usize {
    state.iter().fold(0, |sum, (_, count)| sum + count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut state = read_input("example.txt");

        for _ in 0..18 {
            let new_state = next_day(state);
            state = new_state;
        }

        assert_eq!(state.len(), 26);

        for _ in 18..80 {
            let new_state = next_day(state);
            state = new_state;
        }
        assert_eq!(state.len(), 5934);
    }

    #[test]
    fn example1a() {
        let vec_state = read_input("example.txt");

        let mut state = vec2hashy(vec_state);

        for _ in 0..18 {
            let new_state = next_day3(state);
            state = new_state.clone();
        }
        assert_eq!(count_size(&state), 26);
        for _ in 18..80 {
            let new_state = next_day3(state);
            state = new_state.clone();
        }
        assert_eq!(count_size(&state), 5934);
    }

    #[test]
    fn part1() {
        let mut state = read_input("input.txt");

        for _ in 0..80 {
            let new_state = next_day2(state);
            state = new_state;
        }
        assert_eq!(state.len(), 373378);
    }

    #[test]
    fn example2() {
        let vec_state = read_input("example.txt");

        let mut state = vec2hashy(vec_state);
        for _ in 0..256 {
            let new_state = next_day3(state);
            state = new_state;
        }
        assert_eq!(count_size(&state), 26_984_457_539);
    }

    #[test]
    fn part2() {
        let vec_state = read_input("input.txt");

        let mut state = vec2hashy(vec_state);
        for _ in 0..256 {
            let new_state = next_day3(state);
            state = new_state;
        }
        assert_eq!(count_size(&state), 1682576647495);
    }
}
