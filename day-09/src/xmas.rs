use fxhash::FxHashSet;

mod sums;

use sums::Sums;

#[derive(Debug)]
pub(crate) struct Xmas {
    data: Vec<u64>,
}

impl From<Vec<u64>> for Xmas {
    fn from(data: Vec<u64>) -> Self {
        Self { data }
    }
}

impl Xmas {
    pub(crate) fn find_invalid_number(&self, window_size: usize) -> Option<(u64, usize)> {
        let mut nums = FxHashSet::with_capacity_and_hasher(window_size, Default::default());

        for num in &self.data[..window_size] {
            nums.insert(num);
        }

        for (i, num) in self.data[window_size..].iter().enumerate() {
            let pair_num = nums.iter().find(|&&&first_part| {
                if let Some(second_part) = num.checked_sub(first_part) {
                    nums.contains(&second_part)
                } else {
                    false
                }
            });

            if pair_num.is_none() {
                return Some((*num, i + window_size));
            }

            nums.remove(&self.data[i]);
            nums.insert(num);
        }

        None
    }

    pub(crate) fn find_encryption_weakness(&self, window_size: usize) -> Option<u64> {
        let (target, idx) = self.find_invalid_number(window_size)?;

        let sums: Sums = self.data[0..idx].into();

        for size in 2..(idx - 1) {
            for l in 0..(idx - size) {
                let r = l + size;

                if sums.sum(l, r) == target {
                    let target_slice = &self.data[l..=r];

                    return Some(target_slice.iter().min()? + target_slice.iter().max()?);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_example() {
        let xmas: Xmas = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ]
        .into();

        assert_eq!(xmas.find_invalid_number(5), Some((127, 14)));
    }

    #[test]
    fn test_second_example() {
        let xmas: Xmas = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ]
        .into();

        assert_eq!(xmas.find_encryption_weakness(5), Some(62));
    }
}
