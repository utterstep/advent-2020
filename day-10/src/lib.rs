use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{parse_file, Part, Solver};

mod jolts;

use jolts::Jolts;

#[derive(Debug)]
pub struct Solution {
    jolts: Jolts,
}

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        let jolts = parse_file(input_file)?.into();

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
