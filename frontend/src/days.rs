use std::{error::Error, fmt, str::FromStr};

use advent_utils::{Part, Solver};
use paste::paste;
use variant_count::VariantCount;

macro_rules! days {
    ($($day: tt),*) => {
        paste! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, VariantCount)]
            pub(crate) enum Day {
                $([<Day $day>],)*
            }

            impl fmt::Display for Day {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "Day {:02}", (*self).day_number())
                }
            }

            impl Day {
                pub(crate) const DAYS: [Day; Day::VARIANT_COUNT] = [
                    $(Day::[<Day $day>],)*
                ];

                pub(crate) fn implemented_parts(self) -> Vec<Part> {
                    match self {
                        $(Day::[<Day $day>] => [<day_ $day>]::Solution::implemented_parts(),)*
                    }
                }

                pub(crate) fn solve(self, part: Part, data: &str) -> Result<String, Box<dyn Error>> {
                    Ok(match self {
                        $(Day::[<Day $day>] => [<day_ $day>]::Solution::from_str(data)?.solve(part),)*
                    })
                }

                pub(crate) fn day_number(self) -> usize {
                    self as usize + 1
                }
            }
        }
    };
}

days!(01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14);
