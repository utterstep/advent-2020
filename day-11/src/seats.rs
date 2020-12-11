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

        Ok(Self { seats: seats.into_iter().flatten().collect(), width: width.ok_or(GridParseError::EmptyGrid)? })
    }
}

impl Grid {
    fn make_step(&mut self) -> bool {
        let mut something_changed = false;

        let mut seats = self.seats.clone();

        for (seat_no, current) in self.seats.iter().enumerate() {
            let current = match current {
                None => continue,
                Some(seat) => seat,
            };

            let x = seat_no % self.width;

            let indexes = [
                // HEAD
                if x == 0 { None } else { seat_no.checked_sub(self.width + 1) },
                seat_no.checked_sub(self.width),
                if x == self.width - 1 { None } else { seat_no.checked_sub(self.width - 1) },
                if x == 0 { None } else { seat_no.checked_sub(1) },
                // TAIL
                if x == self.width - 1 { None } else { Some(seat_no + 1) },
                if x == 0 { None } else { Some(seat_no + self.width - 1) },
                Some(seat_no + self.width),
                if x == self.width - 1 { None } else { Some(seat_no + self.width + 1) },
            ];

            let neighbours = indexes
                .iter()
                .filter_map(|no| no
                    .map(|no| self.seats.get(no).map(|&seat| seat))
                    .flatten()
                    .flatten()
                )
                .filter(|seat| matches!(seat, Seat::Occupied))
                .count();

            match current {
                Seat::Empty => {
                    if neighbours == 0 {
                        seats[seat_no].replace(Seat::Occupied);

                        something_changed = true;
                    }
                },
                Seat::Occupied => {
                    if neighbours >= 4 {
                        seats[seat_no].replace(Seat::Empty);

                        something_changed = true;
                    }
                }
            }
        }

        self.seats = seats;

        something_changed
    }

    pub(crate) fn run_simulation_simple(&mut self) {
        while self.make_step() {}
    }

    pub(crate) fn seats(&self) -> impl Iterator<Item = &Seat> {
        self.seats
            .iter()
            .filter_map(|s| s.as_ref())
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let seats = self
            .seats
            .chunks_exact(self.width)
            .map(|row| row.iter().map(|seat| match seat {
                None => '.',
                Some(Seat::Empty) => 'L',
                Some(Seat::Occupied) => '#',
            }).collect::<String>())
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
    fn test_example() {
        let mut grid: Grid = indoc!("L.LL.LL.LL
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

        println!("Before:\n{}\n\n", grid);

        grid.run_simulation();

        let occupied_seats = grid.seats().filter(|&&seat| matches!(seat, Seat::Occupied)).count();

        assert_eq!(occupied_seats, 37);
    }
}
