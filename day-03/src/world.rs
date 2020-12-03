use std::{ops::Index, str::FromStr};

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug)]
struct Toboggan {
    position: (usize, usize),
    velocity: (usize, usize),
}

impl Toboggan {
    pub fn new(velocity: (usize, usize)) -> Self {
        Self {
            position: (0, 0),
            velocity,
        }
    }

    pub fn make_move(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    }
}

#[derive(Debug)]
pub(crate) struct Map {
    width: usize,
    trees: Vec<Vec<bool>>,
}

impl Index<(usize, usize)> for Map {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.trees[y][x % self.width]
    }
}

#[derive(Debug)]
pub(crate) struct World<'a> {
    map: &'a Map,
    toboggan: Toboggan,
}

impl<'a> World<'a> {
    pub fn new(map: &'a Map, velocity: (usize, usize)) -> Self {
        let toboggan = Toboggan::new(velocity);

        Self { map, toboggan }
    }

    pub fn count_trees(mut self) -> usize {
        let mut trees = 0;

        while self.toboggan.position.1 < self.map.trees.len() {
            if self.map[self.toboggan.position] {
                trees += 1;
            }

            self.toboggan.make_move()
        }

        trees
    }
}

#[derive(Debug, Display, Error)]
/// Map parsing error
pub enum MapParseError {
    /// Map has inconsistent width
    InconsistentWidth,
    /// Map has unknown symbols
    UnknownSymbol(char),
    /// Map is empty
    EmptyMap,
}

const TREE: char = '#';
const EMPTY_SPACE: char = '.';

impl FromStr for Map {
    type Err = MapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        TREE => Ok(true),
                        EMPTY_SPACE => Ok(false),
                        c => Err(MapParseError::UnknownSymbol(c)),
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if trees.windows(2).any(|pair| pair[0].len() != pair[1].len()) {
            return Err(MapParseError::InconsistentWidth);
        }

        return Ok(Self {
            width: trees.get(0).ok_or(MapParseError::EmptyMap)?.len(),
            trees,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_example() {
        let map = indoc!(
            "..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#"
        )
        .parse::<Map>()
        .unwrap();

        let world = World::new(&map, (3, 1));

        assert_eq!(world.count_trees(), 7);

        const PART_TWO_VELICITIES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        let part_two_encounters = PART_TWO_VELICITIES.iter().map(|velocity| {
            let world = World::new(&map, *velocity);

            world.count_trees()
        });

        assert_eq!(part_two_encounters.product::<usize>(), 336);
    }
}
