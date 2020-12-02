use std::{convert::TryFrom, error::Error};

use advent_utils::{parse_file, Part, Solver};

mod password;

use password::Record;

#[derive(Debug)]
pub struct Solution {
    records: Vec<Record>,
}

impl TryFrom<String> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let records = parse_file(value)?;

        Ok(Self { records })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => format!(
                "there are {} valid passwords (using letters count)",
                self.records
                    .iter()
                    .filter(|r| r.is_valid_by_count())
                    .count(),
            ),
            Part::Two => format!(
                "there are {} valid passwords (using positional method)",
                self.records
                    .iter()
                    .filter(|r| r.is_valid_positional())
                    .count(),
            ),
        }
    }

    fn day_number() -> u32 {
        2
    }
}
