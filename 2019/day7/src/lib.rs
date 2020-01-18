#[cfg(test)]
mod tests {

    extern crate day5;
    use day5::read_program;
    use std::collections::HashSet;
    use std::collections::VecDeque;

    fn run(memory: &Vec<isize>, mut inputs: Vec<isize>) -> isize {
        let mut uniques = HashSet::new();
        inputs.retain(|e| uniques.insert(e.clone()));
        if 5 != uniques.len() {
            return 0;
        }

        let mut input_a = VecDeque::new();
        let mut input_b = VecDeque::new();
        let mut input_c = VecDeque::new();
        let mut input_d = VecDeque::new();
        let mut input_e = VecDeque::new();

        input_a.push_back(0);

        input_a.push_back(inputs[0]);
        input_b.push_back(inputs[1]);
        input_c.push_back(inputs[2]);
        input_d.push_back(inputs[3]);
        input_e.push_back(inputs[4]);

        loop {
            match read_program(&mut memory.clone(), &mut input_a.clone().into()) {
                Ok(v) => input_b.push_front(v),
                Err(v) => input_b.push_front(v),
            }
            match read_program(&mut memory.clone(), &mut input_b.clone().into()) {
                Ok(v) => input_c.push_front(v),
                Err(v) => input_c.push_front(v),
            }
            match read_program(&mut memory.clone(), &mut input_c.clone().into()) {
                Ok(v) => input_d.push_front(v),
                Err(v) => input_d.push_front(v),
            }
            match read_program(&mut memory.clone(), &mut input_d.clone().into()) {
                Ok(v) => input_e.push_front(v),
                Err(v) => input_e.push_front(v),
            }
            match read_program(&mut memory.clone(), &mut input_e.clone().into()) {
                Ok(v) => return v,
                Err(v) => input_a.push_front(v),
            }
        }
    }

    #[test]
    fn part2() {
        let memory = vec![
            3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 38, 63, 76, 89, 106, 187, 268, 349, 430, 99999,
            3, 9, 1001, 9, 5, 9, 102, 3, 9, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 101, 4, 9, 9, 102, 3,
            9, 9, 101, 4, 9, 9, 1002, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 101, 5, 9, 9, 1002, 9,
            4, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 1002, 9, 5, 9, 4, 9, 99, 3, 9, 1001, 9, 5, 9, 1002,
            9, 5, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
            3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9,
            1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2,
            9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9,
            9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
            3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
            1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
            1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2,
            9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9,
            4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9,
            3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
            1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1,
            9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2,
            9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
            3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
            101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99,
        ];

        let mut max_output = 0;

        for a in 5..=9 {
            for b in 5..=9 {
                for c in 5..=9 {
                    for d in 5..=9 {
                        for e in 5..=9 {
                            let inputs = vec![a, b, c, d, e];
                            let output = run(&mut memory.clone(), inputs);
                            if output > max_output {
                                max_output = output;
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(2645740, max_output);
    }
    #[test]
    fn example_2_1() {
        // Max thruster signal 139629729 (from phase setting sequence 9,8,7,6,5):
        let mut memory = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];

        let inputs = vec![9, 8, 7, 6, 5];
        assert_eq!(139629729, run(&mut memory, inputs))
    }

    #[test]
    fn example_2_2() {
        // Max thruster signal 139629729 (from phase setting sequence 9,8,7,6,5):
        let mut memory = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];

        let inputs = vec![9, 7, 8, 5, 6];
        assert_eq!(139629729, run(&mut memory, inputs))
    }

    #[test]
    fn part1() {
        let memory = vec![
            3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 38, 63, 76, 89, 106, 187, 268, 349, 430, 99999,
            3, 9, 1001, 9, 5, 9, 102, 3, 9, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 101, 4, 9, 9, 102, 3,
            9, 9, 101, 4, 9, 9, 1002, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 101, 5, 9, 9, 1002, 9,
            4, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 1002, 9, 5, 9, 4, 9, 99, 3, 9, 1001, 9, 5, 9, 1002,
            9, 5, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
            3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9,
            1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2,
            9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9,
            9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
            3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
            1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
            1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2,
            9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9,
            4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9,
            3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
            1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1,
            9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2,
            9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
            3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
            101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99,
        ];

        let mut max_output = 0;

        for a in 0..=4 {
            for b in 0..=4 {
                for c in 0..=4 {
                    for d in 0..=4 {
                        for e in 0..=4 {
                            let inputs = vec![a, b, c, d, e];
                            let output = run(&mut memory.clone(), inputs);
                            if output > max_output {
                                max_output = output;
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(21860, max_output);
    }

    #[test]
    fn example_1_1() {
        // Max thruster signal 43210 (from phase setting sequence 4,3,2,1,0):
        let mut memory = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];

        let inputs = vec![4, 3, 2, 1, 0];
        assert_eq!(43210, run(&mut memory, inputs))
    }

    #[test]
    fn example_1_2() {
        // Max thruster signal 54321 (from phase setting sequence 0,1,2,3,4)
        let mut memory = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let inputs = vec![0, 1, 2, 3, 4];

        assert_eq!(54321, run(&mut memory, inputs))
    }

    #[test]
    fn example_1_3() {
        // Max thruster signal 65210 (from phase setting sequence 1,0,4,3,2)
        let mut memory = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let inputs = vec![1, 0, 4, 3, 2];

        assert_eq!(65210, run(&mut memory, inputs))
    }
}
