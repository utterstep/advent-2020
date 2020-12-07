use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{read_file, Part, Solver};

mod bags;

use bags::{BagsRestriction, RestrictionsGraph};

const NEEDLE_COLOR: &str = "shiny gold";

#[derive(Debug)]
pub struct Solution {
    raw_data: String,
}

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self {
            raw_data: read_file(input_file)?,
        })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        7
    }

    fn solve(&self, part: Part) -> String {
        let graph: RestrictionsGraph<'_> = self
            .raw_data
            .lines()
            .map(BagsRestriction::try_from)
            .map(Result::unwrap)
            .collect();

        match part {
            Part::One => format!(
                "there are {} possible containers for {} bag",
                graph.count_possible_containers(NEEDLE_COLOR),
                NEEDLE_COLOR,
            ),
            Part::Two => format!(
                "{} bag must contain {} other bags",
                NEEDLE_COLOR,
                graph.count_children_bags(NEEDLE_COLOR),
            ),
        }
    }
}
