use std::{error::Error, num::ParseIntError, str::FromStr};

use advent_utils::{Part, Solver};
use displaydoc::Display;
use thiserror::Error;

#[derive(Debug)]
pub struct Solution {
    min_departure_time: u64,
    timetable: Vec<Option<u64>>,
}

fn closest_bus<'a>(
    min_departure_time: u64,
    timetable: impl Iterator<Item = &'a u64>,
) -> Option<u64> {
    timetable
        .min_by_key(|&id| {
            let div = min_departure_time / id;
            let rem = min_departure_time % id;

            if rem > 0 {
                (div + 1) * id
            } else {
                div * id
            }
        })
        .copied()
}

#[derive(Debug, Display, Error)]
/// Error while parsing input data
pub enum TimetableParseError {
    /// Departure time not specified
    NoDepartureTime,
    /// Timetable not specified
    NoTimetable,
    /// Error while partsing departure time: {0}
    DepartureTimeInvalid(#[from] ParseIntError),
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let mut lines = input_data.lines();
        let departure_time = lines
            .next()
            .ok_or(TimetableParseError::NoDepartureTime)?
            .parse()?;
        let timetable = lines
            .next()
            .ok_or(TimetableParseError::NoTimetable)?
            .split(',')
            .map(|data| data.parse::<u64>().ok())
            .collect();

        Ok(Self {
            min_departure_time: departure_time,
            timetable,
        })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        13
    }

    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let min_id = closest_bus(
                    self.min_departure_time,
                    self.timetable.iter().filter_map(|id| id.as_ref()),
                );

                let min_id = match min_id {
                    Some(id) => id,
                    None => return "no suitable buses :(".to_owned(),
                };

                let div = self.min_departure_time / min_id;
                let rem = self.min_departure_time % min_id;

                let departure = if rem > 0 {
                    (div + 1) * min_id
                } else {
                    div * min_id
                };

                format!(
                    "you will depart in bus №{} at {}. Answer is {}",
                    min_id,
                    departure,
                    min_id * (departure - self.min_departure_time)
                )
            }
            Part::Two => unimplemented!(),
        }
    }

    fn implemented_parts() -> Vec<Part> {
        vec![Part::One]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closest() {
        let ids: [u64; 5] = [7, 13, 59, 31, 19];

        assert_eq!(closest_bus(939, ids.iter()), Some(59));
    }
}
