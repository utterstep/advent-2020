use std::{convert::TryFrom, error::Error};

use advent_utils::{read_file, Part, Solver};

mod passport;

use passport::Passport;

#[derive(Debug)]
pub struct Solution {
    passports: Vec<Passport>,
}

impl TryFrom<String> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let passports = read_file(value)?
            .split("\n\n")
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        Ok(Self { passports })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => format!(
                "{} passports contains required fields",
                self.passports
                    .iter()
                    .filter(|p| p.contains_required_fields())
                    .inspect(|p| println!("{:#?}", p))
                    .count()
            ),
            Part::Two => format!(
                "{} passports are valid",
                self.passports
                    .iter()
                    .filter(|p| p.is_valid())
                    .inspect(|p| println!("{:#?}", p))
                    .count()
            ),
        }
    }

    fn day_number() -> u32 {
        4
    }
}
