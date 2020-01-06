#[cfg(test)]
mod tests {

    use OpCode::*;
    use ParameterMode::*;

    enum OpCode {
        Add = 1,
        Multiply = 2,
        Input = 3,
        Output = 4,
        Halt = 99,
    }

    #[derive(PartialEq, Debug)]
    enum ParameterMode {
        Position = 0,
        Immediate = 1,
    }

    fn get_instruction_size(input: isize) -> usize {
        match input % 100 {
            x if x == Add as isize => 4,
            x if x == Multiply as isize => 4,
            x if x == Input as isize => 2,
            x if x == Output as isize => 2,
            x if x == Halt as isize => 1,
            _ => panic!("Invalid OpCode"),
        }
    }

    fn decode_opcode(input: isize) -> (isize, ParameterMode, ParameterMode, ParameterMode) {
        let opcode = input % 100;
        let parameter1 = if (input / 100) % 10 == 1 {
            Immediate
        } else {
            Position
        };
        let parameter2 = if (input / 1000) % 10 == 1 {
            Immediate
        } else {
            Position
        };
        let parameter3 = if (input / 10000) % 10 == 1 {
            Immediate
        } else {
            Position
        };
        (opcode, parameter1, parameter2, parameter3)
    }

    fn process_instruction(memory: &mut [isize], instruction: &[isize]) -> bool {
        let (opcode, p1_mode, p2_mode, p3_mode) = decode_opcode(instruction[0]);
        match opcode {
            x if x == Add as isize => {
                let p1 = instruction[1];
                let p2 = instruction[2];
                let p3 = instruction[3];
                assert!(p3_mode == Position);

                memory[p3 as usize] = match p2_mode {
                    Immediate => p2,
                    Position => memory[p2 as usize],
                } + match p1_mode {
                    Immediate => p1,
                    Position => memory[p1 as usize],
                };

                false
            }
            x if x == Multiply as isize => {
                let p1 = instruction[1];
                let p2 = instruction[2];
                let p3 = instruction[3];
                assert!(p3_mode == Position);

                memory[p3 as usize] = match p2_mode {
                    Immediate => p2,
                    Position => memory[p2 as usize],
                } * match p1_mode {
                    Immediate => p1,
                    Position => memory[p1 as usize],
                };
                false
            }
            x if x == Input as isize => {
                assert!(p1_mode == Position);
                assert!(p2_mode == Position);
                assert!(p3_mode == Position);

                let p1 = instruction[1] as usize;

                println!("Please enter a one:");
                let input = String::from("1");
                // io::stdin().read_line(&mut input).expect("Bad Input");
                // &input.trim_end_matches('\n');
                println!("You entered: {}", &input);
                memory[p1] = isize::from_str_radix(&input, 10).expect("bad");
                false
            }
            x if x == Output as isize => {
                assert!(p2_mode == Position);
                assert!(p3_mode == Position);

                let p1 = instruction[1] as usize;
                match p1_mode {
                    Immediate => {
                        println!("Value: {}", p1);
                    }
                    Position => {
                        println!("Value:{}", memory[p1]);
                    }
                }
                false
            }
            x if x == Halt as isize => true,
            _ => panic!("Invalid OpCode"),
        }
    }

    fn read_program(memory: &mut [isize]) {
        let mut memory_location = 0;
        let mut instruction = [0, 0, 0, 0];

        let mut instruction_size = get_instruction_size(memory[memory_location]);
        instruction[0..instruction_size]
            .copy_from_slice(&memory[memory_location..memory_location + instruction_size]);
        memory_location += get_instruction_size(memory[memory_location]);

        while !process_instruction(memory, &instruction) {
            instruction_size = get_instruction_size(memory[memory_location]);
            instruction[0..instruction_size]
                .copy_from_slice(&memory[memory_location..memory_location + instruction_size]);
            memory_location += instruction_size;
        }
    }

    #[test]
    fn part1() {
        let mut memory = [
            3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1101, 32, 43, 225, 101, 68, 192, 224,
            1001, 224, -160, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 2, 224, 1, 223, 224, 223,
            1001, 118, 77, 224, 1001, 224, -87, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 6, 224,
            1, 223, 224, 223, 1102, 5, 19, 225, 1102, 74, 50, 224, 101, -3700, 224, 224, 4, 224,
            1002, 223, 8, 223, 1001, 224, 1, 224, 1, 223, 224, 223, 1102, 89, 18, 225, 1002, 14,
            72, 224, 1001, 224, -3096, 224, 4, 224, 102, 8, 223, 223, 101, 5, 224, 224, 1, 223,
            224, 223, 1101, 34, 53, 225, 1102, 54, 10, 225, 1, 113, 61, 224, 101, -39, 224, 224, 4,
            224, 102, 8, 223, 223, 101, 2, 224, 224, 1, 223, 224, 223, 1101, 31, 61, 224, 101, -92,
            224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 4, 224, 1, 223, 224, 223, 1102, 75, 18,
            225, 102, 48, 87, 224, 101, -4272, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 7,
            224, 1, 224, 223, 223, 1101, 23, 92, 225, 2, 165, 218, 224, 101, -3675, 224, 224, 4,
            224, 1002, 223, 8, 223, 101, 1, 224, 224, 1, 223, 224, 223, 1102, 8, 49, 225, 4, 223,
            99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247,
            1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106,
            0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280,
            1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0,
            300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999,
            1107, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 329, 1001, 223, 1, 223, 1007, 677,
            226, 224, 1002, 223, 2, 223, 1006, 224, 344, 1001, 223, 1, 223, 108, 677, 226, 224,
            102, 2, 223, 223, 1006, 224, 359, 1001, 223, 1, 223, 7, 226, 226, 224, 1002, 223, 2,
            223, 1005, 224, 374, 101, 1, 223, 223, 107, 677, 677, 224, 1002, 223, 2, 223, 1006,
            224, 389, 1001, 223, 1, 223, 1007, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 404,
            1001, 223, 1, 223, 1107, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 419, 1001, 223,
            1, 223, 108, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 434, 1001, 223, 1, 223, 1108,
            226, 677, 224, 1002, 223, 2, 223, 1006, 224, 449, 1001, 223, 1, 223, 1108, 677, 226,
            224, 102, 2, 223, 223, 1005, 224, 464, 1001, 223, 1, 223, 107, 226, 226, 224, 102, 2,
            223, 223, 1006, 224, 479, 1001, 223, 1, 223, 1008, 226, 226, 224, 102, 2, 223, 223,
            1005, 224, 494, 101, 1, 223, 223, 7, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 509,
            101, 1, 223, 223, 8, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 524, 1001, 223, 1,
            223, 1007, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 539, 101, 1, 223, 223, 1008,
            677, 677, 224, 1002, 223, 2, 223, 1006, 224, 554, 101, 1, 223, 223, 1108, 677, 677,
            224, 102, 2, 223, 223, 1006, 224, 569, 101, 1, 223, 223, 1107, 226, 677, 224, 102, 2,
            223, 223, 1005, 224, 584, 1001, 223, 1, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1006,
            224, 599, 101, 1, 223, 223, 1008, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 614,
            1001, 223, 1, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 629, 101, 1, 223,
            223, 107, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 644, 101, 1, 223, 223, 8, 677,
            677, 224, 102, 2, 223, 223, 1005, 224, 659, 1001, 223, 1, 223, 108, 677, 677, 224,
            1002, 223, 2, 223, 1005, 224, 674, 101, 1, 223, 223, 4, 223, 99, 226,
        ];
        read_program(&mut memory);
        // Prints 5821753
    }

    #[test]
    fn misc() {
        println!("{}", (1002 / 1000) % 10 == 1);
        assert_eq!(decode_opcode(1002), (2, Position, Immediate, Position));
        assert_eq!(decode_opcode(11002), (2, Position, Immediate, Immediate));
        assert_eq!(decode_opcode(11102), (2, Immediate, Immediate, Immediate));
        assert_eq!(decode_opcode(02), (2, Position, Position, Position));
    }

    #[test]
    fn example1() {
        let mut memory = [1002, 4, 3, 4, 33];

        read_program(&mut memory);
        assert_eq!(memory, [1002, 4, 3, 4, 99]);
    }

    #[test]
    fn example2() {
        let mut memory = [1101, 100, -1, 4, 0];

        read_program(&mut memory);
        assert_eq!(memory, [1101, 100, -1, 4, 99]);
    }

    #[test]
    fn example3() {
        let mut memory = [3, 0, 4, 0, 99];

        read_program(&mut memory);
    }

    #[test]
    fn example_input() {
        let mut memory = [3, 0, 99];

        read_program(&mut memory);
        assert_eq!(memory, [1, 0, 99]);
    }

    #[test]
    fn example_output() {
        let mut memory = [4, 1, 99];

        read_program(&mut memory);
        assert_eq!(memory, [4, 1, 99]);
    }

    #[test]
    fn example_add() {
        let mut memory = [1, 0, 0, 0, 99];

        read_program(&mut memory);
        assert_eq!(memory, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn example_multiply() {
        let mut memory = [2, 3, 0, 3, 99];

        read_program(&mut memory);
        assert_eq!(memory, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn example_day2() {
        let mut memory = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        read_program(&mut memory);
        assert_eq!(memory, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }
}
