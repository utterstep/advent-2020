use std::{error::Error, str::FromStr};

use advent_utils::{parse_raw_data, Part, Solver};

mod jolts;

use jolts::Jolts;

#[derive(Debug)]
pub struct Solution {
    jolts: Jolts,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let jolts = parse_raw_data(input_data)?.into();

        Ok(Self { jolts })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        10
    }

    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => format!("diff count product is: {}", self.jolts.count_diffs()),
            Part::Two => format!(
                "there are {} possible paths",
                self.jolts.clone().count_paths(0)
            ),
        }
    }
}
