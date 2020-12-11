use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{read_file, Part, Solver};

mod seats;

use seats::{Grid, Seat};

#[derive(Debug)]
pub struct Solution {
    grid: Grid,
}

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        let grid = read_file(input_file)?.parse()?;

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
                grid.run_simulation();

                let occupied_seats = grid.seats().filter(|&&seat| matches!(seat, Seat::Occupied)).count();

                format!("there are {} occupied seats after simulation", occupied_seats)
            },
            Part::Two => unimplemented!(),
        }
    }
}
