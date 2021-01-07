use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

mod questionnaire;

use questionnaire::Group;

#[derive(Debug)]
pub struct Solution {
    groups: Vec<Group>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let groups = input_data
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
