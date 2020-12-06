use std::str::FromStr;

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug)]
pub(crate) struct Questionnaire(u32);

#[derive(Debug)]
pub(crate) struct Group {
    forms: Vec<Questionnaire>,
}

#[derive(Debug, Display, Error)]
pub(crate) enum QuestionParseError {
    /// Got invalid answer specifier: {0}
    InvalidAnswer(char),
}

impl FromStr for Questionnaire {
    type Err = QuestionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().try_fold(0u32, |answers, answer| {
            let answer_number = u32::from(answer)
                .checked_sub(u32::from('a'))
                .ok_or(QuestionParseError::InvalidAnswer(answer))?;

            let answer_bit = 1u32
                .checked_shl(answer_number)
                .ok_or(QuestionParseError::InvalidAnswer(answer))?;

            Ok(answers | answer_bit)
        })?))
    }
}

impl From<Vec<Questionnaire>> for Group {
    fn from(forms: Vec<Questionnaire>) -> Self {
        Self { forms }
    }
}

impl FromStr for Group {
    type Err = QuestionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.lines()
            .map(Questionnaire::from_str)
            .collect::<Result<Vec<_>, _>>()?
            .into())
    }
}

impl Group {
    pub(crate) fn count_yes_any(&self) -> u32 {
        self.forms
            .iter()
            .map(|answers| answers.0)
            .fold(0, |acc, answer| acc | answer)
            .count_ones()
    }

    pub(crate) fn count_yes_all(&self) -> u32 {
        self.forms
            .iter()
            .map(|answers| answers.0)
            .fold(u32::MAX, |acc, answer| acc & answer)
            .count_ones()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_examples() {
        let one_group = indoc!(
            "abcx
            abcy
            abcz"
        );

        let group: Group = one_group.parse().unwrap();

        assert_eq!(group.count_yes_any(), 6);

        let five_groups = indoc!(
            "abc

            a
            b
            c

            ab
            ac

            a
            a
            a
            a

            b"
        );

        let groups = five_groups
            .split("\n\n")
            .map(|group_raw| group_raw.parse::<Group>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(groups.iter().map(|g| g.count_yes_any()).sum::<u32>(), 11);
    }
}
