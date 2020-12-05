use std::{convert::TryFrom, error::Error};

use advent_utils::{parse_file, Part, Solver};

mod boarding_pass;

use boarding_pass::BoardingPass;

#[derive(Debug)]
pub struct Solution {
    passes: Vec<BoardingPass>,
}

impl TryFrom<String> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        let passes = parse_file(path)?;

        Ok(Self { passes })
    }
}

fn sum_up_to_n(n: u32) -> u32 {
    (n * (n + 1)) >> 1
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        let ids = self.passes.iter().map(BoardingPass::id).collect::<Vec<_>>();

        match part {
            Part::One => {
                let max_id = *ids.iter().max().expect("no passes provided");

                format!("max ID is {}", max_id)
            }
            Part::Two => {
                let max_id = *ids.iter().max().expect("no passes provided");
                let min_id = *ids.iter().min().expect("no passes provided");

                let expected_sum = sum_up_to_n(max_id) - sum_up_to_n(min_id - 1);
                let real_sum = ids.iter().sum::<u32>();

                format!("missing pass ID is {}", expected_sum - real_sum)
            }
        }
    }

    fn day_number() -> u32 {
        5
    }
}
