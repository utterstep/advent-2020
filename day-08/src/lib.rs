use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

mod vm;

use vm::{Operation, Vm};

#[derive(Debug)]
pub struct Solution {
    vm: Vm,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            vm: input_data.parse()?,
        })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        8
    }

    fn solve(&self, part: Part) -> String {
        let mut vm = self.vm.clone();

        match part {
            Part::One => {
                vm.detect_loop();

                format!(
                    "accumulator state before entering infinite loop: {}",
                    vm.accumulator(),
                )
            }
            Part::Two => {
                let mut instructions_to_flip = vm
                    .instructions()
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, instruction)| {
                        if matches!(instruction.operation(), Operation::Nop | Operation::Jmp) {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_iter();
                let mut previous_change = None;

                let flip_operation = |vm: &mut Vm, idx| {
                    if let Some(i) = vm.get_mut_instruction(idx) {
                        i.change_operation(match i.operation() {
                            Operation::Jmp => Operation::Nop,
                            Operation::Nop => Operation::Jmp,
                            other => other,
                        });
                    }
                };

                while vm.detect_loop() {
                    if let Some(idx) = previous_change {
                        flip_operation(&mut vm, idx);
                    }

                    vm.reset();

                    match instructions_to_flip.next() {
                        Some(idx) => {
                            flip_operation(&mut vm, idx);

                            previous_change.replace(idx);
                        }
                        None => return "unable to make required change".to_owned(),
                    }
                }

                format!("vm halted with accumulator state {}", vm.accumulator())
            }
        }
    }
}
