use std::fs;
use std::path::Path;
use std::ops::{Index, IndexMut};

struct IntcodeProgram {
    input: Vec<usize>,
    memory: Vec<usize>,
    instruction_pointer: usize,
}

impl IntcodeProgram {
    fn new(input: &str) -> IntcodeProgram {
        let input: Vec<usize> = input.split(',')
            .map(|item| item.parse::<usize>().unwrap())
            .collect();
        let memory = input.clone();
        IntcodeProgram { input, memory, instruction_pointer: 0 }
    }

    fn reset(&mut self, noun: usize, verb: usize) {
        self.memory = self.input.clone();
        self.instruction_pointer = 0;
        self[1] = noun;
        self[2] = verb;
    }

    fn output(&self) -> usize {
        self[0]
    }

    fn run(&mut self) {
        while self.do_instruction() {}
    }

    fn do_instruction(&mut self) -> bool {
        let opcode = self[self.instruction_pointer];
        let result = match opcode {
            1 => self.instruction_1(),
            2 => self.instruction_2(),
            99 => self.instruction_99(),
            _ => panic!("Unknown opcode {} at position {}", opcode, self.instruction_pointer)
        };
        result
    }

    fn get_parameter(&self, index: usize) -> usize {
        self[self.instruction_pointer + index]
    }

    fn instruction_1(&mut self) -> bool {
        let result_address = self.get_parameter(3);
        self[result_address] = self[self.get_parameter(1)] + self[self.get_parameter(2)];
        self.instruction_pointer += 4;
        true
    }

    fn instruction_2(&mut self) -> bool {
        let result_address = self.get_parameter(3);
        self[result_address] = self[self.get_parameter(1)] * self[self.get_parameter(2)];
        self.instruction_pointer += 4;
        true
    }

    fn instruction_99(&mut self) -> bool {
        self.instruction_pointer += 1;
        false
    }

    #[cfg(test)]
    fn to_string(&self) -> String {
        let strings: Vec<String> = self.memory.iter().map(usize::to_string).collect();
        strings.join(",")
    }
}

impl Index<usize> for IntcodeProgram {
    type Output = usize;

    fn index(&self, address: usize) -> &usize {
        &self.memory[address]
    }
}

impl IndexMut<usize> for IntcodeProgram {
    fn index_mut(&mut self, address: usize) -> &mut usize {
        &mut self.memory[address]
    }
}

fn main() {
    let mut program = IntcodeProgram::new(parse_line("src/day/2/input.txt.txt").trim());
    part_1(&mut program);
    part_2(&mut program);
}

fn part_1(program: &mut IntcodeProgram) {
    program.reset(12, 2);
    program.run();
    println!("Answer 1 = {}", program.output());
}

fn part_2(program: &mut IntcodeProgram) {
    let size = program.input.len();
    for noun in 0..size {
        for verb in 0..size {
            program.reset(noun, verb);
            program.run();
            if program.output() == 19690720 {
                println!("Answer 2 = {}", 100 * noun + verb);
                return
            }
        }
    }
    panic!("Could not find answer 2!")
}

//TODO move to utils
fn parse_line<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn small_programs() {
        assert_program_result("1,0,0,0,99", "2,0,0,0,99");
        assert_program_result("2,3,0,3,99", "2,3,0,6,99");
        assert_program_result("2,4,4,5,99,0", "2,4,4,5,99,9801");
        assert_program_result("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99");
    }

    fn assert_program_result(input: &str, expected_result: &str) {
        let mut program = IntcodeProgram::new(input);
        program.run();
        assert_eq!(program.to_string(), expected_result);
    }
}