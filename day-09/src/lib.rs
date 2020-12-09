use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{parse_file, Part, Solver};

mod xmas;

use xmas::Xmas;

#[derive(Debug)]
pub struct Solution {
    xmas: Xmas,
}

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        let xmas = parse_file(input_file)?.into();

        Ok(Self { xmas })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        9
    }

    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => format!(
                "first invalid number is {}",
                self.xmas
                    .find_invalid_number(25)
                    .expect("no invalid numbers!")
                    .0,
            ),
            Part::Two => format!(
                "encryption weakness is {}",
                self.xmas
                    .find_encryption_weakness(25)
                    .expect("no weaknesses!")
            ),
        }
    }
}
