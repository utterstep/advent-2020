use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{read_file, Part, Solver};

mod questionnaire;

use questionnaire::Group;

#[derive(Debug)]
pub struct Solution {
    groups: Vec<Group>,
}

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        let raw_data = read_file(input_file)?;
        let groups = raw_data
            .split("\n\n")
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { groups })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        6
    }

    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => format!(
                "sum of 'yes' counts in all groups (ANY): {}",
                self.groups.iter().map(|g| g.count_yes_any()).sum::<u32>()
            ),
            Part::Two => format!(
                "sum of 'yes' counts in all groups (ALL): {}",
                self.groups.iter().map(|g| g.count_yes_all()).sum::<u32>()
            ),
        }
    }
}
