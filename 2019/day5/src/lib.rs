use OpCode::*;
use ParameterMode::*;

pub struct IntcodeComputer {
    relative_base: usize,
}

enum OpCode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    RelativeBaseOffset = 9,
    Halt = 99,
}

#[derive(PartialEq, Debug)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

enum Status {
    Running,
    Halted,
    Paused,
}
use Status::*;

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

fn get_instruction_size(input: isize) -> usize {
    match input % 100 {
        x if x == Add as isize => 4,
        x if x == Multiply as isize => 4,
        x if x == Input as isize => 2,
        x if x == Output as isize => 2,
        x if x == Halt as isize => 1,
        x if x == JumpIfTrue as isize => 3,
        x if x == JumpIfFalse as isize => 3,
        x if x == LessThan as isize => 4,
        x if x == Equals as isize => 4,
        x if x == RelativeBaseOffset as isize => 2,
        _ => panic!(format!("Invalid OpCode ({})", input)),
    }
}

impl IntcodeComputer {
    fn get_parameter(&self, mode: ParameterMode, instruction: isize, memory: &[isize]) -> isize {
        match mode {
            Immediate => instruction,
            Position => memory[instruction as usize],
            Relative => memory[instruction as usize + self.relative_base],
        }
    }
    fn process_instruction(
        &mut self,
        memory: &mut [isize],
        instruction: &[isize],
        input: &mut Vec<isize>,
    ) -> (Status, Option<usize>, Option<isize>) {
        let (opcode, p1_mode, p2_mode, p3_mode) = decode_opcode(instruction[0]);
        match opcode {
            x if x == Add as isize => {
                assert!(p3_mode == Position);

                let p1 = self.get_parameter(p1_mode, instruction[1], &memory);
                let p2 = self.get_parameter(p2_mode, instruction[2], &memory);
                let p3 = instruction[3];

                memory[p3 as usize] = p2 + p1;

                (Running, None, None)
            }
            x if x == Multiply as isize => {
                assert!(p3_mode == Position);

                let p1 = self.get_parameter(p1_mode, instruction[1], &memory);
                let p2 = self.get_parameter(p2_mode, instruction[2], &memory);
                let p3 = instruction[3];

                memory[p3 as usize] = p2 * p1;

                (Running, None, None)
            }
            x if x == Input as isize => {
                assert!(p1_mode == Position);
                assert!(p2_mode == Position);
                assert!(p3_mode == Position);

                let p1 = instruction[1] as usize;
                if input.len() == 0 {
                    (Paused, None, None)
                } else {
                    let val = input.pop().expect("Unable to pop");
                    println!("Input value: {}", val);
                    memory[p1] = val;

                    (Running, None, None)
                }
            }
            x if x == Output as isize => {
                assert!(p2_mode == Position);
                assert!(p3_mode == Position);

                let p1 = self.get_parameter(p1_mode, instruction[1], &memory);

                println!("Output Value: {}", p1);

                (Running, None, Some(p1))
            }

            x if x == JumpIfTrue as isize => {
                assert!(p3_mode == Position);

                let p1 = self.get_parameter(p1_mode, instruction[1], &memory);
                let p2 = self.get_parameter(p2_mode, instruction[2], &memory) as usize;

                if p1 != 0 {
                    (Running, Some(p2), None)
                } else {
                    (Running, None, None)
                }
            }

            x if x == JumpIfFalse as isize => {
                assert!(p3_mode == Position);

                let p1 = self.get_parameter(p1_mode, instruction[1], &memory);
                let p2 = self.get_parameter(p2_mode, instruction[2], &memory) as usize;

                if p1 == 0 {
                    (Running, Some(p2), None)
                } else {
                    (Running, None, None)
                }
            }

            x if x == LessThan as isize => {
                assert!(p3_mode == Position);

                let p1 = self.get_parameter(p1_mode, instruction[1], &memory);
                let p2 = self.get_parameter(p2_mode, instruction[2], &memory);
                let p3 = instruction[3] as usize;

                memory[p3] = if p1 < p2 { 1 } else { 0 };

                (Running, None, None)
            }

            x if x == Equals as isize => {
                assert!(p3_mode == Position);

                let p1 = self.get_parameter(p1_mode, instruction[1], &memory);
                let p2 = self.get_parameter(p2_mode, instruction[2], &memory);
                let p3 = instruction[3] as usize;

                memory[p3] = if p1 == p2 { 1 } else { 0 };

                (Running, None, None)
            }

            x if x == RelativeBaseOffset as isize => {
                let p1 = self.get_parameter(p1_mode, instruction[1], &memory) as usize;
                println!("Congrats! youve hit the RelativeBaseOffset!");
                self.relative_base += p1;
                (Running, None, None)
            }

            x if x == Halt as isize => {
                assert!(p1_mode == Position);
                assert!(p2_mode == Position);
                assert!(p3_mode == Position);

                (Halted, None, None)
            }
            _ => panic!(format!("Invalid OpCode ({})", opcode)),
        }
    }

    pub fn read_program(memory: &mut [isize], mut input: &mut Vec<isize>) -> Result<isize, isize> {
        color_backtrace::install();

        let mut intcode = IntcodeComputer { relative_base: 0 };

        let mut output = 0;
        let mut memory_location = 0;
        let mut instruction = [0, 0, 0, 0];

        let mut instruction_size = get_instruction_size(memory[memory_location]);
        instruction[0..instruction_size]
            .copy_from_slice(&memory[memory_location..memory_location + instruction_size]);

        loop {
            let (status, addr_override, output_var) =
                &mut intcode.process_instruction(memory, &instruction, &mut input);

            match status {
                Halted => return Ok(output),
                Paused => return Err(output),
                Running => {}
            };

            match output_var {
                Some(v) => output = *v,
                _ => (),
            };

            match addr_override {
                Some(v) => {
                    memory_location = *v;
                }
                None => {
                    memory_location += instruction_size;
                }
            };

            instruction_size = get_instruction_size(memory[memory_location]);
            instruction[0..instruction_size]
                .copy_from_slice(&memory[memory_location..memory_location + instruction_size]);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::ParameterMode::{Immediate, Position};
    use crate::{decode_opcode, IntcodeComputer};

    #[test]
    fn part2_example1() {
        let memory = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        // Using position mode, consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).

        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![7])
        );
        assert_eq!(
            Ok(1),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![8])
        );
        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![9])
        );
    }

    #[test]
    fn part2_example2() {
        let memory = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        // Using position mode, consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).

        assert_eq!(
            Ok(1),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![7])
        );
        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![8])
        );
        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![9])
        );
    }

    #[test]
    fn part2_example3() {
        let memory = [3, 3, 1108, -1, 8, 3, 4, 3, 99];
        // Using immediate mode, consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).

        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![7])
        );
        assert_eq!(
            Ok(1),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![8])
        );
        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![9])
        );
    }

    #[test]
    fn part2_example4() {
        let memory = [3, 3, 1107, -1, 8, 3, 4, 3, 99];
        // Using immediate mode, consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).

        assert_eq!(
            Ok(1),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![7])
        );
        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![8])
        );
        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![9])
        );
    }

    #[test]
    fn part2_example5() {
        let memory = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        //The above example program uses an input instruction to ask for a single number.
        // The program will then output 999 if the input value is below 8, output 1000 if
        // the input value is equal to 8, or output 1001 if the input value is greater than 8.

        assert_eq!(
            Ok(999),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![7])
        );
        assert_eq!(
            Ok(1000),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![8])
        );
        assert_eq!(
            Ok(1001),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![9])
        );
    }

    #[test]
    fn part1_2() {
        let memory = [
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
        assert_eq!(
            Ok(5821753),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![1])
        );
        assert_eq!(
            Ok(11956381),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![5])
        );
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

        IntcodeComputer::read_program(&mut memory, &mut vec![0]);
        assert_eq!(memory, [1002, 4, 3, 4, 99]);
    }

    #[test]
    fn example2() {
        let mut memory = [1101, 100, -1, 4, 0];

        IntcodeComputer::read_program(&mut memory, &mut vec![0]);
        assert_eq!(memory, [1101, 100, -1, 4, 99]);
    }

    #[test]
    fn example3() {
        let mut memory = [3, 0, 4, 0, 99];

        IntcodeComputer::read_program(&mut memory, &mut vec![0]);
    }

    #[test]
    fn example_input() {
        let mut memory = [3, 0, 99];

        IntcodeComputer::read_program(&mut memory, &mut vec![122]);
        assert_eq!(memory, [122, 0, 99]);
    }

    #[test]
    fn example_output() {
        let mut memory = [4, 1, 99];

        IntcodeComputer::read_program(&mut memory, &mut vec![0]);
        assert_eq!(memory, [4, 1, 99]);
    }

    #[test]
    fn example_add() {
        let mut memory = [1, 0, 0, 0, 99];

        IntcodeComputer::read_program(&mut memory, &mut vec![0]);
        assert_eq!(memory, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn example_multiply() {
        let mut memory = [2, 3, 0, 3, 99];

        IntcodeComputer::read_program(&mut memory, &mut vec![0]);
        assert_eq!(memory, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn example_day2() {
        let mut memory = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        IntcodeComputer::read_program(&mut memory, &mut vec![0]);
        assert_eq!(memory, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }
}
