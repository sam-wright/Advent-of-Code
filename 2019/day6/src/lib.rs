#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    fn construct_map(input: &'static str, memory: &mut HashMap<&str, &'static str>) {
        let relationships = input.split('\n');
        for relationship in relationships {
            let v: Vec<&str> = relationship.trim().split_terminator(')').collect();

            memory.entry(v[1].into()).or_insert(v[0]);
        }
    }

    fn measure_map(memory: &HashMap<&str, &'static str>) -> i32 {
        let mut distance = 0;
        for (_child, parent) in memory {
            let mut parent = parent;
            distance += 1;
            while parent != &"COM" {
                distance += 1;
                parent = &memory[parent];
            }
        }

        distance
    }

    // Implement a simple simultaneous BFS from YOU and SAN,
    // where they intersect, measure your distance.
    fn measure_transfers(memory: &HashMap<&str, &'static str>) -> i32 {
        let mut you_exp = Vec::new();
        let mut san_exp = Vec::new();
        let mut you = &memory[&"YOU"];
        let mut san = &memory[&"SAN"];
        let mut distance = 0;

        you_exp.push(you);
        san_exp.push(san);
        let intersection;
        loop {
            if san != &"COM" {
                san = &memory[san];
                san_exp.push(san);
            }

            if you != &"COM" {
                you = &memory[you];
                you_exp.push(you);
            }

            // Check if the exploration lists have overlapped
            if san_exp.contains(&you) {
                intersection = you;
                break;
            } else if you_exp.contains(&san) {
                intersection = san;
                break;
            }
        }

        for thing in you_exp {
            distance += 1;
            if thing == intersection {
                break;
            }
        }
        for thing in san_exp {
            distance += 1;
            if thing == intersection {
                break;
            }
        }

        distance - 2
    }

    // Unforunately need to _coerce_ this String into a 'static String interpretation...
    fn string_to_static_str(s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }

    #[test]
    fn part1() {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::open("input.txt").expect("Unable to open file");
        let mut contents = String::new();
        let mut memory = HashMap::new();

        file.read_to_string(&mut contents)
            .expect("Cannot read file");

        construct_map(string_to_static_str(contents), &mut memory);

        assert_eq!(254447, measure_map(&memory));
    }

    #[test]
    fn part2() {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::open("input.txt").expect("Unable to open file");
        let mut contents = String::new();
        let mut memory = HashMap::new();

        file.read_to_string(&mut contents)
            .expect("Cannot read file");

        construct_map(string_to_static_str(contents), &mut memory);

        assert_eq!(445, measure_transfers(&memory));
    }
    #[test]
    fn example1() {
        let mut memory = HashMap::new();

        construct_map(
            "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L",
            &mut memory,
        );

        assert_eq!(42, measure_map(&memory));
    }

    #[test]
    fn example2() {
        let mut memory = HashMap::new();

        construct_map(
            "COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L
            K)YOU
            I)SAN",
            &mut memory,
        );

        assert_eq!(4, measure_transfers(&memory));
    }
}
