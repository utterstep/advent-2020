use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

mod seats;

use seats::{Grid, Seat};

#[derive(Debug)]
pub struct Solution {
    grid: Grid,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let grid = input_data.parse()?;

        Ok(Self { grid })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        11
    }

    fn solve(&self, part: Part) -> String {
        let mut grid = self.grid.clone();

        match part {
            Part::One => {
                grid.run_simulation_simple();

                let occupied_seats = grid
                    .seats()
                    .filter(|&&seat| matches!(seat, Seat::Occupied))
                    .count();

                format!(
                    "there are {} occupied seats after simulation",
                    occupied_seats
                )
            }
            Part::Two => {
                grid.run_simulation_complex();

                let occupied_seats = grid
                    .seats()
                    .filter(|&&seat| matches!(seat, Seat::Occupied))
                    .count();

                format!(
                    "there are {} occupied seats after complex simulation",
                    occupied_seats
                )
            }
        }
    }
}
