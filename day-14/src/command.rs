use std::{num::ParseIntError, str::FromStr};

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Command {
    SetMask { or_mask: u64, and_mask: u64 },
    Write { address: usize, value: u64 },
}

#[derive(Debug, Display, Error)]
pub(crate) enum ParseCommandError {
    /// Got unknown command
    UnknownCommand,
    /// Unknown char in mask: {0}
    UnknownMaskChar(char),
    /// Invalid mask length,
    InvalidMaskLength,
    /// Invalid command format
    InvalidFormat,
    /// Failed to parse int: {0}
    ParseIntError(#[from] ParseIntError),
}

const BIT_LENGTH: usize = 35;

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut operands = s.split(" = ");

        if s.starts_with("mask") {
            let mask = operands.nth(1).ok_or(ParseCommandError::InvalidFormat)?;

            let (and_mask, or_mask) = mask.chars().enumerate().try_fold(
                (std::u64::MAX, 0),
                |(and_mask, or_mask), (idx, chr)| match chr {
                    '0' => {
                        let bit = 1
                            << BIT_LENGTH
                                .checked_sub(idx)
                                .ok_or(ParseCommandError::InvalidMaskLength)?;

                        Ok((and_mask ^ bit, or_mask))
                    }
                    '1' => {
                        let bit = 1 << (BIT_LENGTH - idx);

                        Ok((and_mask ^ bit, or_mask ^ bit))
                    }
                    'X' => Ok((and_mask, or_mask)),
                    other => Err(ParseCommandError::UnknownMaskChar(other)),
                },
            )?;

            Ok(Self::SetMask { and_mask, or_mask })
        } else if s.starts_with("mem") {
            let address_spec = operands.next().ok_or(ParseCommandError::InvalidFormat)?;
            let value = operands
                .next()
                .ok_or(ParseCommandError::InvalidFormat)?
                .parse()?;

            let address = address_spec
                .split('[')
                .nth(1)
                .ok_or(ParseCommandError::InvalidFormat)?;
            if let Some(address) = address.strip_suffix(']') {
                let address = address.parse()?;

                Ok(Self::Write { address, value })
            } else {
                Err(ParseCommandError::InvalidFormat)
            }
        } else {
            Err(ParseCommandError::UnknownCommand)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parse() {
        let cmd: Command = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
            .parse()
            .unwrap();

        assert_eq!(
            cmd,
            Command::SetMask {
                and_mask: 0b1111111111111111111111111111111111111111111111111111111110111101,
                or_mask: 64
            }
        );
    }
}
