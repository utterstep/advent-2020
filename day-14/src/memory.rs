use std::collections::BTreeMap;

use crate::command::Command;

#[derive(Debug)]
pub(crate) struct Memory {
    and_mask: u64,
    or_mask: u64,
    data: BTreeMap<usize, u64>,
}

impl Memory {
    pub(crate) fn new() -> Self {
        Self {
            and_mask: u64::MAX,
            or_mask: 0,
            data: Default::default(),
        }
    }

    pub(crate) fn process_command(&mut self, command: &Command) {
        match command {
            &Command::Write { address, value } => {
                let value = (value & self.and_mask) | self.or_mask;

                self.data.insert(address, value);
            },
            &Command::SetMask { and_mask, or_mask } => {
                self.and_mask = and_mask;
                self.or_mask = or_mask;
            },
        }
    }

    pub(crate) fn data(&self) -> &BTreeMap<usize, u64> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_example() {
        let commands = indoc!(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0"
        )
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Command>, _>>()
        .unwrap();

        let mut mem = Memory::new();
        commands.iter().for_each(|c| mem.process_command(c));

        assert_eq!(mem.data().get(&7), Some(&101));
        assert_eq!(mem.data().get(&8), Some(&64));
        assert_eq!(mem.data().len(), 2);
    }
}
