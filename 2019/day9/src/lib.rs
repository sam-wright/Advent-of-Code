use OpCode::*;
use ParameterMode::*;

use std::collections::HashMap;

pub struct IntcodeComputer {
    relative_base: usize,
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
            Position => {
                if self.memory.contains_key(&(*&instruction as usize)) {
                    *self.memory.entry(*&instruction as usize).or_insert(0)
                } else {
                    panic!(format!("get_parameter:: Invalid key:{}", instruction))
                }
            }
            Relative => {
                if self
                    .memory
                    .contains_key(&((instruction + self.relative_base as isize) as usize))
                {
                    *self
                        .memory
                        .entry((instruction + self.relative_base as isize) as usize)
                        .or_insert(0)
                } else {
                    panic!(format!(
                        "get_parameter:: Invalid key:{}",
                        instruction + self.relative_base as isize
                    ))
                }
            }
        }
    }

    fn process_instruction(
        &mut self,
        instruction: &[isize],
        input: &mut Vec<isize>,
    ) -> (Status, Option<usize>, Option<isize>) {
        let (opcode, p1_mode, p2_mode, p3_mode) = decode_opcode(instruction[0]);
        match opcode {
            x if x == Add as isize => {
                assert!(p3_mode == Position);

                let p1 = self.get_parameter(p1_mode, instruction[1]);
                let p2 = self.get_parameter(p2_mode, instruction[2]);
                let p3 = instruction[3];

                let e = self.memory.entry(*&p3 as usize).or_insert(0);
                *e = p2 + p1;

                (Running, None, None)
            }
            x if x == Multiply as isize => {
                assert!(p3_mode == Position);

                let p1 = self.get_parameter(p1_mode, instruction[1]);
                let p2 = self.get_parameter(p2_mode, instruction[2]);
                let p3 = instruction[3];

                let e = self.memory.entry(*&p3 as usize).or_insert(0);
                *e = p2 * p1;

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

                    let e = self.memory.entry(*&p1 as usize).or_insert(0);
                    *e = val;

                    (Running, None, None)
                }
            }
            x if x == Output as isize => {
                assert!(p2_mode == Position);
                assert!(p3_mode == Position);

                let p1 = self.get_parameter(p1_mode, instruction[1]);

                println!("Output Value: {}", p1);

                (Running, None, Some(p1))
            }

            x if x == JumpIfTrue as isize => {
                assert!(p3_mode == Position);

                let p1 = self.get_parameter(p1_mode, instruction[1]);
                let p2 = self.get_parameter(p2_mode, instruction[2]) as usize;

                if p1 != 0 {
                    (Running, Some(p2), None)
                } else {
                    (Running, None, None)
                }
            }

            x if x == JumpIfFalse as isize => {
                assert!(p3_mode == Position);

                let p1 = self.get_parameter(p1_mode, instruction[1]);
                let p2 = self.get_parameter(p2_mode, instruction[2]) as usize;

                if p1 == 0 {
                    (Running, Some(p2), None)
                } else {
                    (Running, None, None)
                }
            }

            x if x == LessThan as isize => {
                assert!(p3_mode == Position);

                let p1 = self.get_parameter(p1_mode, instruction[1]);
                let p2 = self.get_parameter(p2_mode, instruction[2]);
                let p3 = instruction[3] as usize;

                let e = self.memory.entry(*&p3 as usize).or_insert(0);
                *e = if p1 < p2 { 1 } else { 0 };

                (Running, None, None)
            }

            x if x == Equals as isize => {
                assert!(p3_mode == Position);

                let p1 = self.get_parameter(p1_mode, instruction[1]);
                let p2 = self.get_parameter(p2_mode, instruction[2]);
                let p3 = instruction[3] as usize;

                let e = self.memory.entry(*&p3 as usize).or_insert(0);
                *e = if p1 == p2 { 1 } else { 0 };

                (Running, None, None)
            }

            x if x == RelativeBaseOffset as isize => {
                let p1 = self.get_parameter(p1_mode, instruction[1]) as usize;
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

    fn load_program_into_memory(&mut self, program: &[isize]) {
        program.iter().enumerate().for_each(|(i, &v)| {
            self.memory.entry(i).or_insert(v);
        })
    }

    fn get_next_chunk(&mut self, memory_location: usize, instruction_size: usize) -> [isize; 4] {
        let mut chunk = [0, 0, 0, 0];

        for i in 0..instruction_size {
            chunk[i] = if self.memory.contains_key(&(memory_location + i)) {
                *self.memory.entry(memory_location + i).or_insert(0)
            } else {
                panic!("get_next_chunk:: Invalid key!")
            };
        }
        chunk
    }

    pub fn read_program(program: &mut [isize], mut input: &mut Vec<isize>) -> Result<isize, isize> {
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
    fn example1_1() {
        //takes no input and produces a copy of itself as output.
        let mut memory = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        IntcodeComputer::read_program(&mut memory, &mut vec![0]).unwrap();
        assert_eq!(
            &[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,],
            &memory
        );
    }

    #[test]
    fn example1_2() {
        //should output a 16-digit number.
        let mut memory = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        // I dont have 18 exabytes...
        // switch memory backing to sparse hashmap
        memory.reserve(18_446_744_073_709_551_615);
        IntcodeComputer::read_program(&mut memory, &mut vec![0]).unwrap();
    }

    #[test]
    fn example1_3() {
        //should output the large number in the middle.
        let mut memory = [104, 1125899906842624, 99];

        assert_eq!(
            Ok(1125899906842624),
            IntcodeComputer::read_program(&mut memory, &mut vec![0])
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
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![1])
        );
        assert_eq!(
            Ok(11956381),
            IntcodeComputer::read_program(&mut memory.clone(), &mut vec![5])
        );
    }
}
