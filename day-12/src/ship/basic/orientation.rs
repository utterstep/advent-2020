use crate::Movement;

#[derive(Debug, Copy, Clone)]
pub(crate) enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    const ORDER: [Orientation; 4] = [
        Orientation::North,
        Orientation::East,
        Orientation::South,
        Orientation::West,
    ];

    const ORIENTATIONS_COUNT: usize = 4;

    fn order(&self) -> usize {
        match self {
            Self::North => 0,
            Self::East => 1,
            Self::South => 2,
            Self::West => 3,
        }
    }

    pub(super) fn process_movement(self, movement: &Movement) -> Self {
        match movement {
            Movement::Left(degrees) => {
                Self::ORDER[(
                    Self::ORIENTATIONS_COUNT + self.order() - degrees.quarters()
                ) % Self::ORIENTATIONS_COUNT]
            },
            Movement::Right(degrees) => {
                Self::ORDER[(self.order() + degrees.quarters()) % Self::ORIENTATIONS_COUNT]
            },
            _ => self,
        }
    }

    pub(super) fn to_pos_diff(self) -> (i64, i64) {
        match self {
            Self::North => (1, 0),
            Self::South => (-1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
        }
    }
}
