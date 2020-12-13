use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{read_file, Part, Solver};

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

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        let data = read_file(input_file)?;
        let mut lines = data.lines();
        let departure_time = lines.next().expect("input is too short").parse()?;
        let timetable = lines
            .next()
            .expect("")
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
                )
                .expect("no buses :(");

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
