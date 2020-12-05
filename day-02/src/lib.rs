use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{parse_file, Part, Solver};

mod password;

use password::Record;

#[derive(Debug)]
pub struct Solution {
    records: Vec<Record>,
}

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        let records = parse_file(input_file)?;

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
