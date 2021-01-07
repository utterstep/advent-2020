use std::{error::Error, str::FromStr};

use advent_utils::{parse_raw_data, Part, Solver};

mod command;
mod memory;

use command::Command;
use memory::Memory;

#[derive(Debug)]
pub struct Solution {
    commands: Vec<Command>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            commands: parse_raw_data(input_data)?,
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
