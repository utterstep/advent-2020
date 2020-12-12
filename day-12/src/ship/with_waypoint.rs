use crate::Movement;

use super::Ship;

#[derive(Debug)]
struct Waypoint {
    north_delta: i64,
    east_delta: i64,
}

impl Waypoint {
    fn new() -> Self {
        Self {
            north_delta: 1,
            east_delta: 10,
        }
    }

    fn rot(&mut self, quarters: usize) {
        let sin = match quarters {
            1 => 1,
            2 => 0,
            3 => -1,
            _ => return,
        };

        let cos = match quarters {
            1 => 0,
            2 => -1,
            3 => 0,
            _ => return,
        };

        let n = self.north_delta;
        let e = self.east_delta;

        self.north_delta = cos * n - sin * e;
        self.east_delta = sin * n + cos * e;
    }

    fn process_movement(&mut self, movement: &Movement) {
        match movement {
            Movement::North(delta) => self.north_delta += delta,
            Movement::South(delta) => self.north_delta -= delta,
            Movement::East(delta) => self.east_delta += delta,
            Movement::West(delta) => self.east_delta -= delta,
            Movement::Left(degrees) => self.rot(4 - degrees.quarters()),
            Movement::Right(degrees) => self.rot(degrees.quarters()),
            _ => {}
        }
    }
}

#[derive(Debug)]
pub(crate) struct WaypointedShip {
    north: i64,
    east: i64,
    waypoint: Waypoint,
}

impl WaypointedShip {
    pub(crate) fn new() -> Self {
        Self {
            north: 0,
            east: 0,
            waypoint: Waypoint::new(),
        }
    }
}

impl Ship for WaypointedShip {
    fn process_movement(&mut self, movement: &Movement) {
        if let Movement::Forward(steps) = movement {
            self.north += self.waypoint.north_delta * steps;
            self.east += self.waypoint.east_delta * steps;
        } else {
            self.waypoint.process_movement(movement);
        }
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
        let movements: Vec<Movement> = indoc!(
            "F10
            N3
            F7
            R90
            F11"
        )
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

        let mut ship = WaypointedShip::new();

        for movement in &movements {
            ship.process_movement(movement);
        }

        println!("{:#?}", ship);

        assert_eq!(ship.manhattan_distance(), 286);
    }
}
