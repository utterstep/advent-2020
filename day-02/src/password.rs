use std::{num::ParseIntError, str::FromStr};

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug)]
pub(crate) struct Password(String);

/// Example: `1-3 a`
///
/// `letter` is `a`, `min_count` is `1`, `max_count` is `3`
#[derive(Debug)]
pub(crate) struct Policy {
    letter: char,
    first_number: usize,
    second_number: usize,
}

/// Example: `1-3 a: abcde`
#[derive(Debug)]
pub(crate) struct Record {
    policy: Policy,
    password: Password,
}

impl Record {
    pub fn is_valid_by_count(&self) -> bool {
        let count = self.password.0.matches(self.policy.letter).count();

        count >= self.policy.first_number && count <= self.policy.second_number
    }

    pub fn is_valid_positional(&self) -> bool {
        let first_letter = self.password.0.chars().nth(self.policy.first_number - 1);
        let second_letter = self.password.0.chars().nth(self.policy.second_number - 1);

        (first_letter == Some(self.policy.letter)) ^ (second_letter == Some(self.policy.letter))
    }
}

/// Failed to parse policy
#[derive(Debug, Display, Error)]
pub(crate) enum PolicyParseError {
    /// Failed to parse number: {0}
    NumberParseError(#[from] ParseIntError),
    /// Unknown policy format
    UnknownFormat,
}

impl FromStr for Policy {
    type Err = PolicyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split(' ');

        let limits = splitted.next().ok_or(PolicyParseError::UnknownFormat)?;

        let mut limits_splitted = limits.split('-');

        let min_count = limits_splitted
            .next()
            .ok_or(PolicyParseError::UnknownFormat)?
            .parse()?;

        let max_count = limits_splitted
            .next()
            .ok_or(PolicyParseError::UnknownFormat)?
            .parse()?;

        let letter = splitted.next().ok_or(PolicyParseError::UnknownFormat)?;

        if letter.len() != 1 {
            return Err(PolicyParseError::UnknownFormat);
        }

        Ok(Self {
            letter: letter.chars().next().expect("invalid codepoint"),
            first_number: min_count,
            second_number: max_count,
        })
    }
}

/// Failed to parse record
#[derive(Debug, Display, Error)]
pub(crate) enum RecordParseError {
    /// Policy parse faliure: {0}
    PolicyParseError(#[from] PolicyParseError),
    /// Failed to parse number: {0}
    NumberParseError(#[from] ParseIntError),
    /// Unknown record format
    UnknownFormat,
}

impl FromStr for Record {
    type Err = RecordParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split(": ");

        let policy = splitted
            .next()
            .ok_or(RecordParseError::UnknownFormat)?
            .parse()?;

        let password = splitted
            .next()
            .ok_or(RecordParseError::UnknownFormat)?
            .to_owned();

        Ok(Self {
            policy,
            password: Password(password),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let record: Record = "1-3 a: abcde".parse().unwrap();

        assert!(record.is_valid_by_count());
        assert!(record.is_valid_positional());

        let record: Record = "1-3 b: cdefg".parse().unwrap();

        assert!(!record.is_valid_by_count());
        assert!(!record.is_valid_positional());

        let record: Record = "2-9 c: ccccccccc".parse().unwrap();

        assert!(record.is_valid_by_count());
        assert!(!record.is_valid_positional());
    }
}
