use std::{convert::TryFrom, error::Error};

use advent_utils::{read_file, Part, Solver};

mod world;

use world::{Map, World};

#[derive(Debug)]
pub struct Solution {
    map: Map,
}

impl TryFrom<String> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let map = read_file(value)?.parse()?;

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
