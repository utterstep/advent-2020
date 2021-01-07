use std::{fmt, str::FromStr};

use displaydoc::Display;
use thiserror::Error;

use smallvec::SmallVec;

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

type NeighboursVec = SmallVec<[usize; 8]>;

#[derive(Debug, Clone)]
pub(crate) struct Grid {
    seats: Vec<Option<Seat>>,
    new_seats: Vec<Option<Seat>>,
    neighbours_simple: Vec<Option<NeighboursVec>>,
    neighbours_complex: Vec<Option<NeighboursVec>>,
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
            new_seats: spare_vec,
            width: width.ok_or(GridParseError::EmptyGrid)?,
            neighbours_simple: Vec::new(),
            neighbours_complex: Vec::new(),
        })
    }
}

const GRID_DIRECTIONS: [(i64, i64); 8] = [
    // TOP
    (-1, -1),
    (0, -1),
    (1, -1),
    // MIDDLE
    (-1, 0),
    (1, 0),
    // BOTTOM
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Debug)]
enum NeighboursMode {
    Simple,
    Complex,
}

impl Grid {
    fn populate_neighbours(&mut self, mode: NeighboursMode) {
        let neighbours_map = match mode {
            NeighboursMode::Simple => &mut self.neighbours_simple,
            NeighboursMode::Complex => &mut self.neighbours_complex,
        };

        if neighbours_map.len() == self.seats.len() {
            return;
        }

        neighbours_map.clear();

        let width = self.width as i64;
        let height = (self.seats.len() as i64) / width;

        type Seats<'a> = &'a [Option<Seat>];

        let (simple_finder, complex_finder);

        let get_neighbours: &dyn Fn(i64, i64, Seats) -> NeighboursVec = match mode {
            NeighboursMode::Simple => {
                simple_finder = |x, y, seats: Seats| {
                    let mut neighbours = SmallVec::new();

                    for (x_diff, y_diff) in GRID_DIRECTIONS.iter() {
                        let x = x + x_diff;
                        let y = y + y_diff;

                        if x < 0 || x >= width || y < 0 || y >= height {
                            continue;
                        }

                        let seat_no = (x + y * width) as usize;

                        if seats[seat_no].is_some() {
                            neighbours.push(seat_no)
                        }
                    }

                    neighbours
                };

                &simple_finder
            },
            NeighboursMode::Complex => {
                complex_finder = |x, y, seats: Seats| {
                    let mut neighbours = SmallVec::new();

                    for (x_diff, y_diff) in GRID_DIRECTIONS.iter() {
                        let mut x = x + x_diff;
                        let mut y = y + y_diff;

                        while !(x < 0 || x >= width || y < 0 || y >= height) {
                            let seat_no = (x + y * width) as usize;

                            if seats[seat_no].is_some() {
                                neighbours.push(seat_no);

                                break;
                            } else {
                                x += x_diff;
                                y += y_diff;
                            }
                        }
                    }

                    neighbours
                };

                &complex_finder
            },
        };

        for y in 0..height {
            for x in 0..width {
                let seat_no = (x + y * width) as usize;

                if self.seats[seat_no].is_none() {
                    neighbours_map.push(None);

                    continue;
                }

                let neighbours = get_neighbours(x, y, &self.seats);

                neighbours_map.push(Some(neighbours));
            }
        }
    }

    fn make_step(&mut self, mode: NeighboursMode, occupation_limit: usize) -> bool {
        let neighbours_map = match mode {
            NeighboursMode::Simple => self.neighbours_simple.as_slice(),
            NeighboursMode::Complex => self.neighbours_complex.as_slice(),
        };

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

                if let Some(idxes) = &neighbours_map[seat_no] {
                    for &idx in idxes {
                        match self.seats[idx] {
                            None | Some(Seat::Empty) => {}
                            Some(Seat::Occupied) => neighbours += 1,
                        }
                    }
                }

                match current {
                    Seat::Empty => {
                        if neighbours == 0 {
                            self.new_seats[seat_no].replace(Seat::Occupied);

                            something_changed = true;
                        }
                    }
                    Seat::Occupied => {
                        if neighbours >= occupation_limit {
                            self.new_seats[seat_no].replace(Seat::Empty);

                            something_changed = true;
                        }
                    }
                }
            }
        }

        self.seats.copy_from_slice(&self.new_seats);

        something_changed
    }

    pub(crate) fn run_simulation_simple(&mut self) {
        self.populate_neighbours(NeighboursMode::Simple);

        while self.make_step(NeighboursMode::Simple, 4) {}
    }

    pub(crate) fn run_simulation_complex(&mut self) {
        self.populate_neighbours(NeighboursMode::Complex);

        while self.make_step(NeighboursMode::Complex, 5) {}
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
