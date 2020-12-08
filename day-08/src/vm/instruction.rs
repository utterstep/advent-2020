use std::{num::ParseIntError, str::FromStr};

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Display, Error)]
pub enum ParseOperationError {
    /// Got unknown operation
    UnknownOperation,
}

impl FromStr for Operation {
    type Err = ParseOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acc" => Ok(Self::Acc),
            "jmp" => Ok(Self::Jmp),
            "nop" => Ok(Self::Nop),
            _ => Err(ParseOperationError::UnknownOperation),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instruction {
    pub(super) operation: Operation,
    pub(super) argument: i32,
}

#[derive(Debug, Display, Error)]
pub enum ParseInstructionError {
    /// Failed to parse operation: {0}
    ParseOperationError(#[from] ParseOperationError),
    /// Failed to parse argument: {0}
    ParseIntError(#[from] ParseIntError),
    /// Unknown instruction format
    UnknownFormat,
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split(' ');

        Ok(Self {
            operation: splitted
                .next()
                .ok_or(ParseInstructionError::UnknownFormat)?
                .parse()?,
            argument: splitted
                .next()
                .ok_or(ParseInstructionError::UnknownFormat)?
                .parse()?,
        })
    }
}

impl Instruction {
    pub fn operation(&self) -> Operation {
        self.operation
    }

    pub fn change_operation(&mut self, new: Operation) {
        self.operation = new;
    }
}
