use std::{convert::TryFrom, error::Error, str::FromStr};

use advent_utils::{Part, Solver};

mod bags;

use bags::{BagsRestriction, RestrictionsGraph};

const NEEDLE_COLOR: &str = "shiny gold";

#[derive(Debug)]
pub struct Solution {
    raw_data: String,
}

impl<'a> FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            raw_data: input_data.to_owned(),
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
