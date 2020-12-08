mod instruction;

use std::str::FromStr;

pub use instruction::{Instruction, Operation, ParseInstructionError};

#[derive(Debug, Clone)]
pub struct Vm {
    accumulator: i32,
    instructions: Vec<Instruction>,
    current: usize,
    visited: Vec<bool>,
}

impl Vm {
    fn make_step(&mut self) {
        if let Some(instruction) = self.instructions.get(self.current) {
            let current_offset = match instruction.operation {
                Operation::Acc => {
                    self.accumulator += instruction.argument;

                    1
                }
                Operation::Jmp => instruction.argument,
                Operation::Nop => 1,
            };

            if current_offset < 0 {
                self.current -= current_offset.abs() as usize;
            } else {
                self.current += current_offset as usize;
            }
        }
    }

    pub fn detect_loop(&mut self) -> bool {
        while self.current != self.instructions.len() {
            if self.visited[self.current] {
                return true;
            }
            self.visited[self.current] = true;

            self.make_step();
        }

        false
    }

    pub fn accumulator(&self) -> i32 {
        self.accumulator
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn get_mut_instruction(&mut self, index: usize) -> Option<&mut Instruction> {
        self.instructions.get_mut(index)
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.current = 0;
        self.visited = vec![false; self.instructions.len()];
    }
}

impl FromStr for Vm {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s.lines().map(str::parse).collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            accumulator: 0,
            current: 0,
            visited: vec![false; instructions.len()],
            instructions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_day_8_examples() {
        let mut vm: Vm = indoc!(
            "nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6"
        )
        .parse()
        .unwrap();

        assert!(vm.detect_loop());
        assert_eq!(vm.accumulator(), 5);

        vm.reset();
        vm.get_mut_instruction(vm.instructions().len() - 2)
            .and_then(|i| {
                i.change_operation(Operation::Nop);
                Some(i)
            });
        assert!(!vm.detect_loop());

        assert_eq!(vm.accumulator(), 8);
    }
}
