use std::collections::BTreeSet;

use fxhash::FxHashMap;

#[derive(Debug, Clone)]
pub(crate) struct Jolts {
    jolts: BTreeSet<u32>,
    max: u32,
    paths: FxHashMap<u32, u64>,
}

impl From<Vec<u32>> for Jolts {
    fn from(jolts: Vec<u32>) -> Self {
        let mut jolts: BTreeSet<u32> = jolts.iter().cloned().collect();
        let max = jolts.iter().next_back().expect("empty bag") + 3;

        jolts.insert(max);

        Self {
            jolts,
            max,
            paths: Default::default(),
        }
    }
}

impl Jolts {
    pub(crate) fn count_diffs(&self) -> u32 {
        let mut ones = 0;
        let mut threes = 0;
        let mut prev = 0;

        for &jolt in self.jolts.iter() {
            if jolt - prev == 1 {
                ones += 1;
            } else if jolt - prev == 3 {
                threes += 1;
            }

            prev = jolt;
        }

        ones * threes
    }

    pub(crate) fn count_paths(&mut self, current: u32) -> u64 {
        if let Some(&paths) = self.paths.get(&current) {
            return paths;
        }

        let paths = {
            if current == self.max {
                1
            } else if current > self.max {
                0
            } else if self.jolts.contains(&current) || current == 0 {
                self.count_paths(current + 1)
                    + self.count_paths(current + 2)
                    + self.count_paths(current + 3)
            } else {
                0
            }
        };

        self.paths.insert(current, paths);

        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_example() {
        let mut jolts: Jolts = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4].into();

        assert_eq!(jolts.count_diffs(), 35);
        assert_eq!(jolts.count_paths(0), 8);
    }

    #[test]
    fn test_medium_example() {
        let mut jolts: Jolts = include_str!("../med.txt")
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into();

        assert_eq!(jolts.count_diffs(), 220);
        assert_eq!(jolts.count_paths(0), 19208);
    }
}
