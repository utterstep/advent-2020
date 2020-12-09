#[derive(Debug)]
pub(super) struct Sums(Vec<u64>);

impl From<&[u64]> for Sums {
    fn from(nums: &[u64]) -> Self {
        Self(
            nums.iter()
                .fold(Vec::with_capacity(nums.len()), |mut sums, &num| {
                    sums.push(sums.last().unwrap_or(&0) + num);

                    sums
                }),
        )
    }
}

impl Sums {
    pub(super) fn sum(&self, l: usize, r: usize) -> u64 {
        debug_assert!(l <= r);

        if l == 0 {
            self.0[r]
        } else {
            self.0[r] - self.0[l - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let input = [1, 2, 3, 4, 5];

        let sums: Sums = input.as_ref().into();

        assert_eq!(input.iter().sum::<u64>(), sums.sum(0, 4),);

        assert_eq!(input[0..=3].iter().sum::<u64>(), sums.sum(0, 3),);

        assert_eq!(input[1..3].iter().sum::<u64>(), sums.sum(1, 2),);

        assert_eq!(input[1..3].iter().sum::<u64>(), sums.sum(1, 2),);
    }
}
