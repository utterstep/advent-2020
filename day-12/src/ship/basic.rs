use crate::Movement;

use super::Ship;

mod orientation;

use orientation::Orientation;

#[derive(Debug)]
pub(crate) struct BasicShip {
    north: i64,
    east: i64,
    orientation: Orientation,
}

impl BasicShip {
    pub(crate) fn new() -> Self {
        Self {
            north: 0,
            east: 0,
            orientation: Orientation::East,
        }
    }
}

impl Ship for BasicShip {
    fn process_movement(&mut self, movement: &Movement) {
        let orientation = self.orientation.process_movement(movement);

        let (n_diff, e_diff) = if let Movement::Forward(value) = movement {
            let (n_diff, e_diff) = self.orientation.to_pos_diff();

            (n_diff * value, e_diff * value)
        } else {
            movement.to_pos_diff()
        };

        self.orientation = orientation;
        self.north += n_diff;
        self.east += e_diff;
    }

    fn manhattan_distance(&self) -> i64 {
        self.north.abs() + self.east.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_example() {
        let movements: Vec<Movement> = indoc!("F10
            N3
            F7
            R90
            F11"
        )
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

        let mut ship = BasicShip::new();

        for movement in &movements {
            ship.process_movement(movement);
        }

        assert_eq!(ship.manhattan_distance(), 25);
    }
}
