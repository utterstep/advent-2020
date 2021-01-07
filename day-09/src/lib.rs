use std::{error::Error, str::FromStr};

use advent_utils::{parse_raw_data, Part, Solver};

mod xmas;

use xmas::Xmas;

#[derive(Debug)]
pub struct Solution {
    xmas: Xmas,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let xmas = parse_raw_data(input_data)?.into();

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
