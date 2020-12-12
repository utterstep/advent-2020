use std::{fmt, str::FromStr};

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Copy, Clone)]
pub(crate) enum Seat {
    Occupied,
    Empty,
}

#[derive(Debug, Display, Error)]
pub(crate) enum GridParseError {
    /// Invalid seat specifier: {0}
    InvalidSeatSpec(char),

    /// Inconsistent grid width
    InconsistentWidth,

    /// Empty grid
    EmptyGrid,
}

#[derive(Debug, Clone)]
pub(crate) struct Grid {
    seats: Vec<Option<Seat>>,
    spare_vec: Vec<Option<Seat>>,
    width: usize,
}

impl FromStr for Grid {
    type Err = GridParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = None;

        let seats: Vec<Vec<_>> = s
            .lines()
            .map(|line| {
                if let Some(old_width) = width.replace(line.len()) {
                    if old_width != line.len() {
                        return Err(GridParseError::InconsistentWidth);
                    }
                }

                line.chars()
                    .map(|c| match c {
                        '.' => Ok(None),
                        'L' => Ok(Some(Seat::Empty)),
                        '#' => Ok(Some(Seat::Occupied)),
                        other => Err(GridParseError::InvalidSeatSpec(other)),
                    })
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;

        let seats = seats.into_iter().flatten().collect::<Vec<_>>();
        let spare_vec = seats.clone();

        Ok(Self {
            seats,
            spare_vec,
            width: width.ok_or(GridParseError::EmptyGrid)?,
        })
    }
}

const GRID_DIRECTIONS: [(i64, i64); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

impl Grid {
    fn make_step_simple(&mut self) -> bool {
        let mut something_changed = false;

        let width = self.width as i64;
        let height = (self.seats.len() as i64) / width;

        for y in 0..height {
            for x in 0..width {
                let seat_no = (x + y * width) as usize;

                let current = match self.seats[seat_no] {
                    None => continue,
                    Some(seat) => seat,
                };

                let mut neighbours = 0;

                for (x_diff, y_diff) in GRID_DIRECTIONS.iter() {
                    let x = x + x_diff;
                    let y = y + y_diff;

                    if x < 0 || x >= width || y < 0 || y >= height {
                        continue;
                    }

                    let seat_no = x + y * width;

                    match self.seats[seat_no as usize] {
                        None => {}
                        Some(Seat::Occupied) => neighbours += 1,
                        Some(Seat::Empty) => {}
                    }
                }

                match current {
                    Seat::Empty => {
                        if neighbours == 0 {
                            self.spare_vec[seat_no].replace(Seat::Occupied);

                            something_changed = true;
                        }
                    }
                    Seat::Occupied => {
                        if neighbours >= 4 {
                            self.spare_vec[seat_no].replace(Seat::Empty);

                            something_changed = true;
                        }
                    }
                }
            }
        }

        self.seats.clone_from(&self.spare_vec);

        something_changed
    }

    fn make_step_complex(&mut self) -> bool {
        let mut something_changed = false;

        let width = self.width as i64;
        let height = (self.seats.len() as i64) / width;

        for y in 0..height {
            for x in 0..width {
                let seat_no = (x + y * width) as usize;

                let current = match self.seats[seat_no] {
                    None => continue,
                    Some(seat) => seat,
                };

                let mut neighbours = 0;

                for (x_diff, y_diff) in GRID_DIRECTIONS.iter() {
                    let mut x = x + x_diff;
                    let mut y = y + y_diff;

                    while !(x < 0 || x >= width || y < 0 || y >= height) {
                        let seat_no = x + y * width;

                        match self.seats[seat_no as usize] {
                            None => {
                                x += x_diff;
                                y += y_diff;

                                continue;
                            }
                            Some(Seat::Occupied) => neighbours += 1,
                            Some(Seat::Empty) => {}
                        }

                        break;
                    }
                }

                match current {
                    Seat::Empty => {
                        if neighbours == 0 {
                            self.spare_vec[seat_no].replace(Seat::Occupied);

                            something_changed = true;
                        }
                    }
                    Seat::Occupied => {
                        if neighbours >= 5 {
                            self.spare_vec[seat_no].replace(Seat::Empty);

                            something_changed = true;
                        }
                    }
                }
            }
        }

        self.seats.clone_from(&self.spare_vec);

        something_changed
    }

    pub(crate) fn run_simulation_complex(&mut self) {
        while self.make_step_complex() {}
    }

    pub(crate) fn run_simulation_simple(&mut self) {
        while self.make_step_simple() {}
    }

    pub(crate) fn seats(&self) -> impl Iterator<Item = &Seat> {
        self.seats.iter().filter_map(|s| s.as_ref())
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let seats = self
            .seats
            .chunks_exact(self.width)
            .map(|row| {
                row.iter()
                    .map(|seat| match seat {
                        None => '.',
                        Some(Seat::Empty) => 'L',
                        Some(Seat::Occupied) => '#',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", seats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_simple_example() {
        let mut grid: Grid = indoc!(
            "L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL"
        )
        .parse()
        .unwrap();

        grid.run_simulation_simple();

        let occupied_seats = grid
            .seats()
            .filter(|&&seat| matches!(seat, Seat::Occupied))
            .count();

        assert_eq!(occupied_seats, 37);
    }

    #[test]
    fn test_complex_example() {
        let mut grid: Grid = indoc!(
            "L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL"
        )
        .parse()
        .unwrap();

        grid.run_simulation_complex();

        let occupied_seats = grid
            .seats()
            .filter(|&&seat| matches!(seat, Seat::Occupied))
            .count();

        assert_eq!(occupied_seats, 26);
    }
}
