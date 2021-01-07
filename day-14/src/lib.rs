use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{parse_file, Part, Solver};

mod command;
mod memory;

use command::Command;
use memory::Memory;

#[derive(Debug)]
pub struct Solution {
    commands: Vec<Command>,
}

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self {
            commands: parse_file(input_file)?,
        })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        14
    }

    fn solve(&self, part: Part) -> String {
        let mut mem = Memory::new();

        match part {
            Part::One => {
                self.commands.iter().for_each(|c| mem.process_command(c));

                format!(
                    "sum of values in memory: {}",
                    mem.data().values().sum::<u64>()
                )
            }
            Part::Two => unimplemented!(),
        }
    }
}
