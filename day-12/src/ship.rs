use crate::Movement;

mod basic;
mod with_waypoint;

pub(crate) use basic::BasicShip;
pub(crate) use with_waypoint::WaypointedShip;

pub(crate) trait Ship {
    fn process_movement(&mut self, movement: &Movement);

    fn manhattan_distance(&self) -> i64;
}
