use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

mod passport;

use passport::Passport;

#[derive(Debug)]
pub struct Solution {
    passports_raw: String,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let passports_raw = input_data.to_owned();

        Ok(Self { passports_raw })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        let passports = self.passports_raw.split("\n\n").map(Passport::new);

        match part {
            Part::One => format!(
                "{} passports contains required fields",
                passports.filter(|p| p.contains_required_fields()).count()
            ),
            Part::Two => format!(
                "{} passports are valid",
                passports.filter(|p| p.is_valid()).count()
            ),
        }
    }

    fn day_number() -> u32 {
        4
    }
}
