use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct BoardingPass {
    row: u32,
    seat: u32,
}

#[derive(Debug)]
pub(crate) struct UnknownFormatError;

impl FromStr for BoardingPass {
    type Err = UnknownFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = s
            .chars()
            .take(7)
            .enumerate()
            .try_fold(0u32, |acc, (idx, chr)| match chr {
                'F' => Ok(acc),
                'B' => Ok(acc | 1 << (6 - idx)),
                _ => Err(UnknownFormatError),
            })?;

        let seat =
            s[7..]
                .chars()
                .take(3)
                .enumerate()
                .try_fold(0u32, |acc, (idx, chr)| match chr {
                    'L' => Ok(acc),
                    'R' => Ok(acc | 1 << (2 - idx)),
                    _ => Err(UnknownFormatError),
                })?;

        Ok(Self { row, seat })
    }
}

impl BoardingPass {
    pub(crate) fn id(&self) -> u32 {
        self.row * 8 + self.seat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        macro_rules! assert_pass_id_eq {
            ($pass: expr, $id: expr) => {
                let pass: BoardingPass = $pass.parse().expect("invalid format");

                assert_eq!(pass.id(), $id);
            };
        }

        assert_pass_id_eq!("FBFBBFFRLR", 357);
        assert_pass_id_eq!("BFFFBBFRRR", 567);
        assert_pass_id_eq!("FFFBBBFRRR", 119);
        assert_pass_id_eq!("BBFFBBFRLL", 820);
    }
}
