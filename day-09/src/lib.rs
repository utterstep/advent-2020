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
            Part::One => match self.xmas.find_invalid_number(25) {
                Some((number, _idx)) => format!("first invalid number is {}", number),
                None => "no invalid numbers".to_owned(),
            },
            Part::Two => match self.xmas.find_encryption_weakness(25) {
                Some(weakness) => format!("encryption weakness is {}", weakness),
                None => "no weaknesses!".to_owned(),
            },
        }
    }
}
