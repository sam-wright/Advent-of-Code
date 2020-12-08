use std::fs::File;
use std::{collections::HashSet, io::Read};

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    nop(i32), // No Op
    acc(i32), // Accumulate
    jmp(i32), // Relative Jump
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ReturnStatus<T> {
    Success(T),
    Duplicate(T),
}

pub fn read_instructions(filename: &str) -> Vec<Instruction> {
    let mut contents = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut contents).unwrap();

    let collection: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();
    let mut instructions = Vec::new();
    for line in collection {
        let words: Vec<&str> = line.split(" ").collect();
        instructions.push(match words[0] {
            "nop" => Instruction::nop(words[1].parse().unwrap()),
            "acc" => Instruction::acc(words[1].parse().unwrap()),
            "jmp" => Instruction::jmp(words[1].parse().unwrap()),
            _ => panic!("Unexpected input instruction"),
        })
    }

    instructions
}

pub fn execute_instructions(instructions: Vec<Instruction>) -> ReturnStatus<i32> {
    let mut accumulator = 0;
    let mut line_number = 0;

    let mut instruction_history = HashSet::new();
    loop {
        // check if we've run this line before
        if instruction_history.contains(&line_number) {
            return ReturnStatus::Duplicate(accumulator);
        }
        instruction_history.insert(line_number.clone());

        match instructions.get(line_number as usize) {
            Some(inst) => match inst {
                Instruction::nop(_) => line_number += 1,
                Instruction::acc(v) => {
                    accumulator += v;
                    line_number += 1;
                }
                Instruction::jmp(v) => line_number += v,
                // _ => panic!("Unexpected instruction"),
            },
            None => break,
        }
    }
    ReturnStatus::Success(accumulator)
}

// Return the line numbers of all instances of an instruction type
pub fn find_instances(instructions: &Vec<Instruction>, target: Instruction) -> Vec<usize> {
    let mut line_numbers = Vec::new();

    for (line_number, line) in instructions.iter().enumerate() {
        if std::mem::discriminant(line) == std::mem::discriminant(&target) {
            line_numbers.push(line_number);
        }
    }

    line_numbers
}

// return a copy of the instructions with the target line swapped
pub fn instruction_swapper(
    instructions: &Vec<Instruction>,
    target_line: usize,
) -> Vec<Instruction> {
    let mut new_inst = Vec::new();
    for (line_number, inst) in instructions.iter().enumerate() {
        if line_number == target_line {
            new_inst.push(match inst {
                Instruction::jmp(v) => Instruction::nop(*v),
                Instruction::nop(v) => Instruction::jmp(*v),
                _ => panic!("Unswappable type!"),
            })
        } else {
            new_inst.push(inst.clone())
        }
    }

    new_inst
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let instructions = read_instructions("example1.txt");
        assert_eq!(
            execute_instructions(instructions),
            ReturnStatus::Duplicate(5)
        );
    }

    #[test]
    fn part_1_solution() {
        let instructions = read_instructions("input.txt");
        assert_eq!(
            execute_instructions(instructions),
            ReturnStatus::Duplicate(2058)
        );
    }

    #[test]
    fn part_2_example() {
        let instructions = read_instructions("example1.txt");
        let mut jumps = find_instances(&instructions, Instruction::jmp(0));
        let mut noops = find_instances(&instructions, Instruction::nop(0));

        jumps.append(&mut noops);

        for attempt in jumps {
            let new_instructions = instruction_swapper(&instructions, attempt);
            let status = execute_instructions(new_instructions);
            match status {
                ReturnStatus::Duplicate(_) => {}
                ReturnStatus::Success(v) => {
                    assert_eq!(v, 8);
                    break;
                }
            }
        }
    }

    #[test]
    fn part_2_solution() {
        let instructions = read_instructions("input.txt");
        let mut jumps = find_instances(&instructions, Instruction::jmp(0));
        let mut noops = find_instances(&instructions, Instruction::nop(0));

        jumps.append(&mut noops);

        for attempt in jumps {
            let new_instructions = instruction_swapper(&instructions, attempt);
            let status = execute_instructions(new_instructions);
            match status {
                ReturnStatus::Duplicate(_) => {}
                ReturnStatus::Success(v) => {
                    assert_eq!(v, 1000);
                    break;
                }
            }
        }
    }
}
