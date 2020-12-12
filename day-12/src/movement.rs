use std::{num::ParseIntError, str::FromStr};

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Degrees(usize);

impl Degrees {
    pub(super) fn quarters(self) -> usize {
        debug_assert!(self.0 % 90 == 0);

        self.0 / 90
    }
}

#[derive(Debug)]
pub(crate) enum Movement {
    Forward(i64),
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(Degrees),
    Right(Degrees),
}

impl Movement {
    pub(super) fn to_pos_diff(&self) -> (i64, i64) {
        match self {
            &Self::North(value) => (value, 0),
            &Self::South(value) => (-value, 0),
            &Self::East(value) => (0, value),
            &Self::West(value) => (0, -value),
            _ => (0, 0),
        }
    }
}

#[derive(Debug, Display, Error)]
pub(crate) enum ParseMovementError {
    /// String is empty
    EmptyStr,
    /// Invalid action: {0},
    InvalidAction(char),
    /// Failed to parse action value: {0},
    ParseIntError(#[from] ParseIntError),
}

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = s.chars().next().ok_or(ParseMovementError::EmptyStr)?;

        match action {
            'N' => Ok(Self::North(s[1..].parse()?)),
            'E' => Ok(Self::East(s[1..].parse()?)),
            'S' => Ok(Self::South(s[1..].parse()?)),
            'W' => Ok(Self::West(s[1..].parse()?)),
            'R' => Ok(Self::Right(Degrees(s[1..].parse()?))),
            'L' => Ok(Self::Left(Degrees(s[1..].parse()?))),
            'F' => Ok(Self::Forward(s[1..].parse()?)),
            other => Err(ParseMovementError::InvalidAction(other)),
        }
    }
}
