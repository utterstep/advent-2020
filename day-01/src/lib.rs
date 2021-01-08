use std::{collections::BTreeSet, error::Error, str::FromStr};

use advent_utils::{parse_raw_data, Part, Solver};

const TARGET_SUM: u32 = 2020;

#[derive(Debug)]
pub struct Solution {
    expenses: BTreeSet<u32>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let expenses = parse_raw_data(input_data)?;

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
                let product = find_two_sum(&self.expenses, TARGET_SUM).map(|(a, b)| a * b);

                match product {
                    Some(product) => format!("target two-sum product is {}", product),
                    None => "couldn't find solution".to_owned(),
                }
            }
            Part::Two => {
                let product = find_three_sum(&self.expenses, TARGET_SUM).map(|(a, b, c)| a * b * c);

                match product {
                    Some(product) => format!("target three-sum product is {}", product),
                    None => "couldn't find solution".to_owned(),
                }
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
