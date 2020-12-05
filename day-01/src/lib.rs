use std::{collections::BTreeSet, convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{parse_file, Part, Solver};

const TARGET_SUM: u32 = 2020;

#[derive(Debug)]
pub struct Solution {
    expenses: BTreeSet<u32>,
}

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        let expenses = parse_file(input_file)?;

        Ok(Self {
            expenses: expenses.into_iter().collect(),
        })
    }
}

fn find_two_sum(numbers: &BTreeSet<u32>, target_sum: u32) -> Option<(u32, u32)> {
    numbers
        .iter()
        .filter_map(|&number| {
            let other = target_sum.checked_sub(number)?;
            if numbers.contains(&other) {
                Some((number, other))
            } else {
                None
            }
        })
        .next()
}

fn find_three_sum(numbers: &BTreeSet<u32>, target_sum: u32) -> Option<(u32, u32, u32)> {
    numbers
        .iter()
        .filter_map(|&other| {
            find_two_sum(numbers, target_sum.checked_sub(other)?).map(|(a, b)| (a, b, other))
        })
        .next()
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let product = find_two_sum(&self.expenses, TARGET_SUM)
                    .map(|(a, b)| a * b)
                    .expect("Part One: Solution unknown");

                format!("target two-sum product is {}", product)
            }
            Part::Two => {
                let product = find_three_sum(&self.expenses, TARGET_SUM)
                    .map(|(a, b, c)| a * b * c)
                    .expect("Part Two: Solution unknown");

                format!("target three-sum product is {}", product)
            }
        }
    }

    fn day_number() -> u32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let expenses = vec![1721, 979, 366, 299, 675, 1456].into_iter().collect();

        assert_eq!(
            find_two_sum(&expenses, TARGET_SUM).map(|(a, b)| a * b),
            Some(514579),
        );

        assert_eq!(
            find_three_sum(&expenses, TARGET_SUM).map(|(a, b, c)| a * b * c),
            Some(241861950),
        );
    }
}
