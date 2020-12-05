use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{read_file, Part, Solver};

mod passport;

use passport::Passport;

#[derive(Debug)]
pub struct Solution {
    passports_raw: String,
}

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        let passports_raw = read_file(input_file)?;

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
