use OpCode::*;
use ParameterMode::*;

use std::collections::HashMap;

const VERBOSE: bool = false;

pub struct IntcodeComputer {
    relative_base: isize,
    memory: HashMap<usize, isize>,
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
    } else if (input / 100) % 10 == 2 {
        Relative
    } else {
        Position
    };

    let parameter2 = if (input / 1000) % 10 == 1 {
        Immediate
    } else if (input / 1000) % 10 == 2 {
        Relative
    } else {
        Position
    };

    let parameter3 = if (input / 10000) % 10 == 1 {
        Immediate
    } else if (input / 10000) % 10 == 2 {
        Relative
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
    fn new() -> Self {
        Self {
            relative_base: 0,
            memory: HashMap::new(),
        }
    }

    fn get_parameter(&mut self, mode: ParameterMode, instruction: isize) -> isize {
        match mode {
            Immediate => instruction,
            Position => *self.memory.entry(instruction as usize).or_insert(0),
            Relative => *self
                .memory
                .entry((instruction + self.relative_base) as usize)
                .or_insert(0),
        }
    }
    fn get_output_parameter(&mut self, mode: ParameterMode, instruction: isize) -> isize {
        match mode {
            Position => instruction,
            Relative => instruction + self.relative_base,
            _ => panic!("Bad parameter mode!"),
        }
    }

    fn process_instruction(
        &mut self,
        instruction: &[isize],
        input: &mut Vec<isize>,
    ) -> (Status, Option<usize>, Option<isize>) {
        let (opcode, p1_mode, p2_mode, p3_mode) = decode_opcode(instruction[0]);
        if VERBOSE {
            print!("Instruction:{:?}\t", &instruction);
            print!("{:?} {:?} {:?}\t", p1_mode, p2_mode, p3_mode,);
        }
        match opcode {
            x if x == Add as isize => {
                assert!(p3_mode != Immediate);

                let p1 = self.get_parameter(p1_mode, instruction[1]);
                let p2 = self.get_parameter(p2_mode, instruction[2]);
                let p3 = self.get_output_parameter(p3_mode, instruction[3]);

                let e = self.memory.entry(p3 as usize).or_insert(0);
                *e = p2 + p1;

                if VERBOSE {
                    println!("Add: memory[{}] = {} + {}", p3, p1, p2);
                }

                (Running, None, None)
            }
            x if x == Multiply as isize => {
                assert!(p3_mode != Immediate);

                let p1 = self.get_parameter(p1_mode, instruction[1]);
                let p2 = self.get_parameter(p2_mode, instruction[2]);
                let p3 = self.get_output_parameter(p3_mode, instruction[3]);

                let e = self.memory.entry(p3 as usize).or_insert(0);
                *e = p2 * p1;

                if VERBOSE {
                    println!("Mul: memory[{}] = {} * {}", p3, p1, p2);
                }

                (Running, None, None)
            }
            x if x == Input as isize => {
                assert!(p1_mode != Immediate);

                let p1 = self.get_output_parameter(p1_mode, instruction[1]);

                if input.len() == 0 {
                    println!("Pausing evaluation, out of inputs");
                    (Paused, None, None)
                } else {
                    let val = input.pop().expect("Unable to pop");
                    // println!("Input value: {}", val);

                    let e = self.memory.entry(p1 as usize).or_insert(0);
                    *e = val;
                    if VERBOSE {
                        println!("Inp: memory[{}] = {}", p1, val);
                    }

                    (Running, None, None)
                }
            }
            x if x == Output as isize => {
                let p1 = self.get_parameter(p1_mode, instruction[1]);

                // if VERBOSE {
                println!("Out: {}", p1);
                // }

                (Running, None, Some(p1))
            }

            x if x == JumpIfTrue as isize => {
                let p1 = self.get_parameter(p1_mode, instruction[1]);
                let p2 = self.get_parameter(p2_mode, instruction[2]);

                if p1 != 0 {
                    if VERBOSE {
                        println!("Jump to {}", p2);
                    }
                    (Running, Some(p2 as usize), None)
                } else {
                    if VERBOSE {
                        println!();
                    }
                    (Running, None, None)
                }
            }

            x if x == JumpIfFalse as isize => {
                let p1 = self.get_parameter(p1_mode, instruction[1]);
                let p2 = self.get_parameter(p2_mode, instruction[2]);

                if p1 == 0 {
                    if VERBOSE {
                        println!("Jump to {}", p2);
                    }
                    (Running, Some(p2 as usize), None)
                } else {
                    if VERBOSE {
                        println!();
                    }
                    (Running, None, None)
                }
            }

            x if x == LessThan as isize => {
                let p1 = self.get_parameter(p1_mode, instruction[1]);
                let p2 = self.get_parameter(p2_mode, instruction[2]);
                let p3 = self.get_output_parameter(p3_mode, instruction[3]);

                if VERBOSE {
                    println!("LT: memory[{}] = ({}<{}={})", p3, p1, p2, p1 < p2);
                }

                let e = self.memory.entry(p3 as usize).or_insert(0);
                *e = if p1 < p2 { 1 } else { 0 };

                (Running, None, None)
            }

            x if x == Equals as isize => {
                assert!(p3_mode != Immediate);

                let p1 = self.get_parameter(p1_mode, instruction[1]);
                let p2 = self.get_parameter(p2_mode, instruction[2]);
                let p3 = self.get_output_parameter(p3_mode, instruction[3]);

                if VERBOSE {
                    println!("EQ: memory[{}] = ({}=={}={})", p3, p1, p2, p1 == p2);
                }

                let e = self.memory.entry(p3 as usize).or_insert(0);
                *e = if p1 == p2 { 1 } else { 0 };

                (Running, None, None)
            }

            x if x == RelativeBaseOffset as isize => {
                let p1 = self.get_parameter(p1_mode, instruction[1]);

                if VERBOSE {
                    println!("RO: Adjusting {} by {}", self.relative_base, p1);
                }

                self.relative_base += p1;
                (Running, None, None)
            }

            x if x == Halt as isize => {
                if VERBOSE {
                    println!("DONE!");
                }

                (Halted, None, None)
            }
            _ => panic!(format!("Invalid OpCode ({})", opcode)),
        }
    }

    fn load_program_into_memory(&mut self, program: &[isize]) {
        program.iter().enumerate().for_each(|(i, &v)| {
            self.memory.entry(i).or_insert(v);
        })
    }

    fn get_next_chunk(&mut self, memory_location: usize, instruction_size: usize) -> [isize; 4] {
        let mut chunk = [0, 0, 0, 0];

        for i in 0..instruction_size {
            chunk[i] = *self.memory.entry(memory_location + i).or_insert(0);
        }
        chunk
    }

    pub fn read_program(program: &[isize], mut input: &mut Vec<isize>) -> Result<isize, isize> {
        color_backtrace::install();

        let mut computer = IntcodeComputer::new();
        computer.load_program_into_memory(&program);

        let mut output = 0;
        let mut memory_location = 0;
        let mut chunk;

        let mut instruction_size = get_instruction_size(computer.memory[&memory_location]);
        chunk = computer.get_next_chunk(memory_location, instruction_size);

        loop {
            let (status, addr_override, output_var) =
                &mut computer.process_instruction(&chunk, &mut input);

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

            instruction_size = get_instruction_size(computer.memory[&memory_location]);
            chunk = computer.get_next_chunk(memory_location, instruction_size);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::IntcodeComputer;

    #[test]
    fn web_test_1() {
        // sum-of-primes: This program takes a single input and produces a single output, the sum of all primes up to the input.
        let mut memory = [
            3, 100, 1007, 100, 2, 7, 1105, -1, 87, 1007, 100, 1, 14, 1105, -1, 27, 101, -2, 100,
            100, 101, 1, 101, 101, 1105, 1, 9, 101, 105, 101, 105, 101, 2, 104, 104, 101, 1, 102,
            102, 1, 102, 102, 103, 101, 1, 103, 103, 7, 102, 101, 52, 1106, -1, 87, 101, 105, 102,
            59, 1005, -1, 65, 1, 103, 104, 104, 101, 105, 102, 83, 1, 103, 83, 83, 7, 83, 105, 78,
            1106, -1, 35, 1101, 0, 1, -1, 1105, 1, 69, 4, 104, 99,
        ];
        assert_eq!(
            Ok(17),
            IntcodeComputer::read_program(&mut memory, &mut vec![10])
        );

        assert_eq!(
            Ok(142913828922),
            IntcodeComputer::read_program(&mut memory, &mut vec![2000000])
        );
    }

    #[test]
    fn part_1() {
        let memory = [
            1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1101, 0, 3, 1000,
            109, 988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65,
            1008, 1000, 2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99,
            4, 0, 104, 0, 99, 4, 17, 104, 0, 99, 0, 0, 1101, 0, 36, 1015, 1102, 1, 387, 1028, 1101,
            24, 0, 1016, 1101, 0, 23, 1008, 1102, 1, 35, 1012, 1102, 1, 554, 1023, 1101, 29, 0,
            1003, 1101, 27, 0, 1011, 1101, 25, 0, 1000, 1101, 0, 38, 1018, 1102, 20, 1, 1019, 1102,
            28, 1, 1005, 1102, 1, 619, 1026, 1102, 1, 22, 1004, 1101, 0, 0, 1020, 1101, 0, 31,
            1009, 1102, 1, 783, 1024, 1102, 1, 33, 1001, 1102, 616, 1, 1027, 1102, 1, 21, 1006,
            1101, 32, 0, 1013, 1102, 39, 1, 1014, 1102, 1, 378, 1029, 1101, 774, 0, 1025, 1102, 1,
            1, 1021, 1102, 30, 1, 1007, 1102, 37, 1, 1002, 1102, 1, 26, 1017, 1101, 0, 557, 1022,
            1102, 1, 34, 1010, 109, 13, 2101, 0, -5, 63, 1008, 63, 23, 63, 1005, 63, 203, 4, 187,
            1105, 1, 207, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -14, 2107, 28, 4, 63, 1005, 63,
            225, 4, 213, 1106, 0, 229, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 10, 1207, -3, 20, 63,
            1005, 63, 245, 1106, 0, 251, 4, 235, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 8, 1205, 3,
            263, 1105, 1, 269, 4, 257, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -9, 1207, -7, 34, 63,
            1005, 63, 287, 4, 275, 1105, 1, 291, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -4, 2102,
            1, -3, 63, 1008, 63, 32, 63, 1005, 63, 311, 1105, 1, 317, 4, 297, 1001, 64, 1, 64,
            1002, 64, 2, 64, 109, 21, 21101, 40, 0, -6, 1008, 1019, 43, 63, 1005, 63, 337, 1106, 0,
            343, 4, 323, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -26, 1202, 7, 1, 63, 1008, 63, 21,
            63, 1005, 63, 365, 4, 349, 1106, 0, 369, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 26,
            2106, 0, 3, 4, 375, 1001, 64, 1, 64, 1105, 1, 387, 1002, 64, 2, 64, 109, -9, 21108, 41,
            40, 3, 1005, 1019, 407, 1001, 64, 1, 64, 1106, 0, 409, 4, 393, 1002, 64, 2, 64, 109,
            13, 1205, -8, 423, 4, 415, 1106, 0, 427, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -19,
            21107, 42, 41, 5, 1005, 1015, 447, 1001, 64, 1, 64, 1106, 0, 449, 4, 433, 1002, 64, 2,
            64, 109, -3, 2102, 1, -5, 63, 1008, 63, 37, 63, 1005, 63, 471, 4, 455, 1105, 1, 475,
            1001, 64, 1, 64, 1002, 64, 2, 64, 109, -2, 1201, 0, 0, 63, 1008, 63, 28, 63, 1005, 63,
            497, 4, 481, 1105, 1, 501, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 8, 2107, 29, -8, 63,
            1005, 63, 521, 1001, 64, 1, 64, 1106, 0, 523, 4, 507, 1002, 64, 2, 64, 109, -3, 1208,
            -3, 30, 63, 1005, 63, 541, 4, 529, 1106, 0, 545, 1001, 64, 1, 64, 1002, 64, 2, 64, 109,
            4, 2105, 1, 9, 1105, 1, 563, 4, 551, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 9, 1206,
            -3, 581, 4, 569, 1001, 64, 1, 64, 1106, 0, 581, 1002, 64, 2, 64, 109, -8, 1201, -9, 0,
            63, 1008, 63, 23, 63, 1005, 63, 605, 1001, 64, 1, 64, 1106, 0, 607, 4, 587, 1002, 64,
            2, 64, 109, 21, 2106, 0, -9, 1106, 0, 625, 4, 613, 1001, 64, 1, 64, 1002, 64, 2, 64,
            109, -35, 2108, 31, 8, 63, 1005, 63, 647, 4, 631, 1001, 64, 1, 64, 1105, 1, 647, 1002,
            64, 2, 64, 109, 2, 1202, 0, 1, 63, 1008, 63, 30, 63, 1005, 63, 667, 1105, 1, 673, 4,
            653, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 17, 21108, 43, 43, -4, 1005, 1016, 691, 4,
            679, 1106, 0, 695, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -14, 1208, -1, 30, 63, 1005,
            63, 711, 1106, 0, 717, 4, 701, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 6, 21101, 44, 0,
            -1, 1008, 1011, 44, 63, 1005, 63, 739, 4, 723, 1105, 1, 743, 1001, 64, 1, 64, 1002, 64,
            2, 64, 109, -15, 2108, 30, 8, 63, 1005, 63, 759, 1106, 0, 765, 4, 749, 1001, 64, 1, 64,
            1002, 64, 2, 64, 109, 27, 2105, 1, 0, 4, 771, 1001, 64, 1, 64, 1105, 1, 783, 1002, 64,
            2, 64, 109, -9, 1206, 6, 795, 1105, 1, 801, 4, 789, 1001, 64, 1, 64, 1002, 64, 2, 64,
            109, 4, 21102, 45, 1, -7, 1008, 1012, 45, 63, 1005, 63, 823, 4, 807, 1105, 1, 827,
            1001, 64, 1, 64, 1002, 64, 2, 64, 109, -14, 21102, 46, 1, 5, 1008, 1010, 43, 63, 1005,
            63, 851, 1001, 64, 1, 64, 1105, 1, 853, 4, 833, 1002, 64, 2, 64, 109, -1, 2101, 0, 1,
            63, 1008, 63, 25, 63, 1005, 63, 873, 1105, 1, 879, 4, 859, 1001, 64, 1, 64, 1002, 64,
            2, 64, 109, 9, 21107, 47, 48, -3, 1005, 1010, 897, 4, 885, 1105, 1, 901, 1001, 64, 1,
            64, 4, 64, 99, 21101, 0, 27, 1, 21101, 915, 0, 0, 1106, 0, 922, 21201, 1, 57526, 1,
            204, 1, 99, 109, 3, 1207, -2, 3, 63, 1005, 63, 964, 21201, -2, -1, 1, 21101, 942, 0, 0,
            1106, 0, 922, 21201, 1, 0, -1, 21201, -2, -3, 1, 21101, 957, 0, 0, 1106, 0, 922, 22201,
            1, -1, -2, 1105, 1, 968, 21202, -2, 1, -2, 109, -3, 2106, 0, 0,
        ];

        assert_eq!(
            Ok(3380552333),
            IntcodeComputer::read_program(&memory, &mut vec![1])
        );

        assert_eq!(
            Ok(78831),
            IntcodeComputer::read_program(&memory, &mut vec![2])
        );
    }

    #[test]
    fn example1_1() {
        //takes no input and produces a copy of itself as output.
        let memory = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        IntcodeComputer::read_program(&memory, &mut vec![0]).unwrap();
    }

    #[test]
    fn example1_2() {
        //should output a 16-digit number.
        let memory = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        assert_eq!(
            Ok(1_219_070_632_396_864),
            IntcodeComputer::read_program(&memory, &mut vec![0])
        );
    }

    #[test]
    fn example1_3() {
        //should output the large number in the middle.
        let memory = [104, 1125899906842624, 99];

        assert_eq!(
            Ok(1125899906842624),
            IntcodeComputer::read_program(&memory, &mut vec![0])
        );
    }

    #[test]
    fn day5_part1_2() {
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
            IntcodeComputer::read_program(&memory.clone(), &mut vec![1])
        );
        assert_eq!(
            Ok(11956381),
            IntcodeComputer::read_program(&memory.clone(), &mut vec![5])
        );
    }

    #[test]
    fn part2_example1() {
        let memory = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        // Using position mode, consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).

        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&memory.clone(), &mut vec![7])
        );
        assert_eq!(
            Ok(1),
            IntcodeComputer::read_program(&memory.clone(), &mut vec![8])
        );
        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&memory.clone(), &mut vec![9])
        );
    }

    #[test]
    fn part2_example2() {
        let memory = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        // Using position mode, consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).

        assert_eq!(
            Ok(1),
            IntcodeComputer::read_program(&memory.clone(), &mut vec![7])
        );
        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&memory.clone(), &mut vec![8])
        );
        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&memory.clone(), &mut vec![9])
        );
    }

    #[test]
    fn part2_example3() {
        let memory = [3, 3, 1108, -1, 8, 3, 4, 3, 99];
        // Using immediate mode, consider whether the input is equal to 8;
        // output 1 (if it is) or 0 (if it is not).

        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&memory.clone(), &mut vec![7])
        );
        assert_eq!(
            Ok(1),
            IntcodeComputer::read_program(&memory.clone(), &mut vec![8])
        );
        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&memory.clone(), &mut vec![9])
        );
    }

    #[test]
    fn part2_example4() {
        let memory = [3, 3, 1107, -1, 8, 3, 4, 3, 99];
        // Using immediate mode, consider whether the input is less than 8;
        // output 1 (if it is) or 0 (if it is not).

        assert_eq!(
            Ok(1),
            IntcodeComputer::read_program(&memory.clone(), &mut vec![7])
        );
        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&memory.clone(), &mut vec![8])
        );
        assert_eq!(
            Ok(0),
            IntcodeComputer::read_program(&memory.clone(), &mut vec![9])
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
            IntcodeComputer::read_program(&memory.clone(), &mut vec![7])
        );
        assert_eq!(
            Ok(1000),
            IntcodeComputer::read_program(&memory.clone(), &mut vec![8])
        );
        assert_eq!(
            Ok(1001),
            IntcodeComputer::read_program(&memory.clone(), &mut vec![9])
        );
    }
}
