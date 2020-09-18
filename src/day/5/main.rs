use std::{fs, fmt};
use std::path::Path;
use std::ops::{Index, IndexMut};
use std::fmt::{Display, Formatter};

struct IntcodeProgram {
    input: isize,
    memory: Vec<isize>,
    instruction_pointer: usize,
    output: Option<isize>,
}

impl IntcodeProgram {
    fn new(program: &str, input: isize) -> IntcodeProgram {
        let memory: Vec<isize> = program.trim().split(',')
            .map(|item| item.parse::<isize>().unwrap())
            .collect();
        IntcodeProgram { input, memory, instruction_pointer: 0, output: None }
    }

    fn run(&mut self) {
        // println!("{}", self);
        while self.do_instruction() {
            // println!("{}", self);
        }
    }

    fn do_instruction(&mut self) -> bool {
        let opcode = self[0] % 100;
        match opcode {
            1 => self.add(),
            2 => self.multiply(),
            3 => self.save(),
            4 => self.output(),
            5 => self.jump_if_true(),
            6 => self.jump_if_false(),
            7 => self.less_than(),
            8 => self.equals(),
            99 => return false,
            _ => panic!("Unknown opcode {} at position {}", opcode, self.instruction_pointer)
        };
        true
    }

    fn get_parameter(&self, index: isize) -> isize {
        let mode = (self[0] / 10isize.pow((1 + index) as u32)) % 10;
        match mode {
            0 => self.memory[self[index] as usize],
            1 => self[index],
            _ => panic!("Unknown mode {} for parameter {} in instruction {}", mode, index, self[0])
        }
    }

    fn get_mut_parameter(&mut self, index: isize) -> &mut isize {
        let address = self[index] as usize;
        &mut self.memory[address]
    }

    fn add(&mut self) {
        *self.get_mut_parameter(3) = self.get_parameter(1) + self.get_parameter(2);
        self.instruction_pointer += 4;
    }

    fn multiply(&mut self) {
        *self.get_mut_parameter(3) = self.get_parameter(1) * self.get_parameter(2);
        self.instruction_pointer += 4;
    }

    fn save(&mut self) {
        *self.get_mut_parameter(1) = self.input;
        self.instruction_pointer += 2;
    }

    fn output(&mut self) {
        self.output = Some(self.get_parameter(1));
        self.instruction_pointer += 2;
    }

    fn jump_if_true(&mut self) {
        self.instruction_pointer = if self.get_parameter(1) != 0 {
            self.get_parameter(2) as usize
        } else {
            self.instruction_pointer + 3
        }
    }

    fn jump_if_false(&mut self) {
        self.instruction_pointer = if self.get_parameter(1) == 0 {
            self.get_parameter(2) as usize
        } else {
            self.instruction_pointer + 3
        }
    }

    fn less_than(&mut self) {
        *self.get_mut_parameter(3) = if self.get_parameter(1) < self.get_parameter(2) { 1 } else { 0 };
        self.instruction_pointer += 4
    }

    fn equals(&mut self) {
        *self.get_mut_parameter(3) = if self.get_parameter(1) == self.get_parameter(2) { 1 } else { 0 };
        self.instruction_pointer += 4
    }
}

impl Display for IntcodeProgram {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut strings: Vec<String> = self.memory.iter().map(isize::to_string).collect();
        strings[self.instruction_pointer] = format!("*{}", strings[self.instruction_pointer]);
        write!(f, "{}", strings.join(","))
    }
}

impl Index<isize> for IntcodeProgram {
    type Output = isize;

    fn index(&self, index: isize) -> &isize {
        let address = (self.instruction_pointer as isize + index) as usize;
        &self.memory[address]
    }
}

impl IndexMut<isize> for IntcodeProgram {
    fn index_mut(&mut self, index: isize) -> &mut isize {
        let address = (self.instruction_pointer as isize + index) as usize;
        &mut self.memory[address]
    }
}

fn main() {
    let program_string = parse_line("src/day/5/input.txt");
    let mut program = IntcodeProgram::new(program_string.as_str(), 1);
    program.run();
    println!("Answer 1 = {}", program.output.unwrap());
    let mut program = IntcodeProgram::new(program_string.as_str(), 5);
    program.run();
    println!("Answer 2 = {}", program.output.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn equals_position_mode() {
        let program = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_program_output(program, 8, 1);
        assert_program_output(program, 7, 0);
    }

    #[test]
    fn less_than_position_mode() {
        let program = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_program_output(program, 7, 1);
        assert_program_output(program, 8, 0);
    }

    #[test]
    fn equals_immediate_mode() {
        let program = "3,3,1108,-1,8,3,4,3,99";
        assert_program_output(program, 8, 1);
        assert_program_output(program, 7, 0);
    }

    #[test]
    fn less_than_immediate_mode() {
        let program = "3,3,1107,-1,8,3,4,3,99";
        assert_program_output(program, 7, 1);
        assert_program_output(program, 8, 0);
    }

    #[test]
    fn jump_position_mode() {
        let program = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        assert_program_output(program, 0, 0);
        assert_program_output(program, 7, 1);
    }

    #[test]
    fn jump_immediate_mode() {
        let program = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_program_output(program, 0, 0);
        assert_program_output(program, 7, 1);
    }

    fn assert_program_output(program: &str, input: isize, expected_output: isize) {
        let mut program = IntcodeProgram::new(program, input);
        program.run();
        assert_eq!(program.output, Some(expected_output));
    }
}

//TODO move to utils
fn parse_line<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path).unwrap()
}