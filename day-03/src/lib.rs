use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

mod world;

use world::{Map, World};

#[derive(Debug)]
pub struct Solution {
    map: Map,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let map = input_data.parse()?;

        Ok(Self { map })
    }
}

const PART_ONE_VELOCITY: (usize, usize) = (3, 1);
const PART_TWO_VELICITIES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let world = World::new(&self.map, PART_ONE_VELOCITY);

                format!("you will encounter {} trees", world.count_trees())
            }
            Part::Two => {
                let part_two_encounters = PART_TWO_VELICITIES.iter().map(|velocity| {
                    let world = World::new(&self.map, *velocity);

                    world.count_trees()
                });

                format!(
                    "encounters count from all routes multiplied: {}",
                    part_two_encounters.product::<usize>()
                )
            }
        }
    }

    fn day_number() -> u32 {
        3
    }
}
