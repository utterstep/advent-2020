use std::{error::Error, fmt, str::FromStr};

use advent_utils::{Part, Solver};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Day {
    Day01 = 1,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Day {:02}", (*self).day_number())
    }
}

impl Day {
    pub(crate) const DAYS: [Day; 14] = [
        Day::Day01,
        Day::Day02,
        Day::Day03,
        Day::Day04,
        Day::Day05,
        Day::Day06,
        Day::Day07,
        Day::Day08,
        Day::Day09,
        Day::Day10,
        Day::Day11,
        Day::Day12,
        Day::Day13,
        Day::Day14,
    ];

    pub(crate) fn implemented_parts(self) -> Vec<Part> {
        match self {
            Day::Day01 => day_01::Solution::implemented_parts(),
            Day::Day02 => day_02::Solution::implemented_parts(),
            Day::Day03 => day_03::Solution::implemented_parts(),
            Day::Day04 => day_04::Solution::implemented_parts(),
            Day::Day05 => day_05::Solution::implemented_parts(),
            Day::Day06 => day_06::Solution::implemented_parts(),
            Day::Day07 => day_07::Solution::implemented_parts(),
            Day::Day08 => day_08::Solution::implemented_parts(),
            Day::Day09 => day_09::Solution::implemented_parts(),
            Day::Day10 => day_10::Solution::implemented_parts(),
            Day::Day11 => day_11::Solution::implemented_parts(),
            Day::Day12 => day_12::Solution::implemented_parts(),
            Day::Day13 => day_13::Solution::implemented_parts(),
            Day::Day14 => day_14::Solution::implemented_parts(),
        }
    }

    pub(crate) fn solve(self, part: Part, data: &str) -> Result<String, Box<dyn Error>> {
        Ok(match self {
            Day::Day01 => day_01::Solution::from_str(data)?.solve(part),
            Day::Day02 => day_02::Solution::from_str(data)?.solve(part),
            Day::Day03 => day_03::Solution::from_str(data)?.solve(part),
            Day::Day04 => day_04::Solution::from_str(data)?.solve(part),
            Day::Day05 => day_05::Solution::from_str(data)?.solve(part),
            Day::Day06 => day_06::Solution::from_str(data)?.solve(part),
            Day::Day07 => day_07::Solution::from_str(data)?.solve(part),
            Day::Day08 => day_08::Solution::from_str(data)?.solve(part),
            Day::Day09 => day_09::Solution::from_str(data)?.solve(part),
            Day::Day10 => day_10::Solution::from_str(data)?.solve(part),
            Day::Day11 => day_11::Solution::from_str(data)?.solve(part),
            Day::Day12 => day_12::Solution::from_str(data)?.solve(part),
            Day::Day13 => day_13::Solution::from_str(data)?.solve(part),
            Day::Day14 => day_14::Solution::from_str(data)?.solve(part),
        })
    }

    pub(crate) fn day_number(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SolutionConfig {
    day: Day,
    part: Part,
}
