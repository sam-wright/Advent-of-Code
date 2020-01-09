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
            //println!("Parent {}, Child {}", &parent, &child);
            let mut parent = parent;
            distance += 1;
            while parent != &"COM" {
                distance += 1;
                parent = &memory[parent];
            }
        }

        dbg!(distance);
        distance
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
}
