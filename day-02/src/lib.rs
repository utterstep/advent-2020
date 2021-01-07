use std::{error::Error, str::FromStr};

use advent_utils::{parse_raw_data, Part, Solver};

mod password;

use password::Record;

#[derive(Debug)]
pub struct Solution {
    records: Vec<Record>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let records = parse_raw_data(input_data)?;

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
