use std::{error::Error, str::FromStr};

use advent_utils::{parse_raw_data, Part, Solver};

mod movement;
mod ship;

use movement::Movement;
use ship::{BasicShip, Ship, WaypointedShip};

#[derive(Debug)]
pub struct Solution {
    movements: Vec<Movement>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let movements = parse_raw_data(input_data)?;

        Ok(Self { movements })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        12
    }

    fn solve(&self, part: Part) -> String {
        let mut basic = BasicShip::new();
        let mut waypointed = WaypointedShip::new();

        let ship: &mut dyn Ship = match part {
            Part::One => &mut basic,
            Part::Two => &mut waypointed,
        };

        for movement in &self.movements {
            ship.process_movement(movement);
        }

        format!("ship distance from zero is: {}", ship.manhattan_distance())
    }
}
