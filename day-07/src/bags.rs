use fxhash::{FxHashMap, FxHashSet};

use std::{
    convert::TryFrom,
    iter::FromIterator,
};

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct BagsRestriction<'a> {
    container: &'a str,
    content: Vec<(u32, &'a str)>,
}

#[derive(Debug, Display, Error)]
pub(crate) enum RestrictionParseError {
    /// Bag container was not specified
    NoContainer,
    /// Bag content was not specified
    NoContent,
}

impl<'a> TryFrom<&'a str> for BagsRestriction<'a> {
    type Error = RestrictionParseError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // example:
        // light red bags contain 1 bright white bag, 2 muted yellow bags.
        let mut data = value.split(" contain ");

        let source = data
            .next()
            .ok_or(RestrictionParseError::NoContainer)?
            .rsplitn(2, ' ')
            .nth(1)
            .ok_or(RestrictionParseError::NoContainer)?;

        let targets = data
            .next()
            .ok_or(RestrictionParseError::NoContent)?
            .split(", ")
            .filter_map(|target| {
                // example: 1 bright white bag
                // example: 2 muted yellow bags.
                // example: no other bags.

                if target.starts_with("no other") {
                    return None;
                }

                let mut parsed = target.splitn(2, ' ');

                let count: u32 = parsed.next()?.parse().ok()?;

                let color = parsed.next()?.rsplitn(2, ' ').nth(1)?;

                Some(Ok((count, color)))
            })
            .collect::<Result<Vec<_>, RestrictionParseError>>()?;

        Ok(Self {
            container: source,
            content: targets,
        })
    }
}

#[derive(Debug, Default)]
struct GraphNode<'a> {
    containers: Vec<&'a str>,
    content: Vec<(u32, &'a str)>,
}

#[derive(Debug)]
pub(crate) struct RestrictionsGraph<'a>(FxHashMap<&'a str, GraphNode<'a>>);

impl<'a> FromIterator<BagsRestriction<'a>> for RestrictionsGraph<'a> {
    fn from_iter<T: IntoIterator<Item = BagsRestriction<'a>>>(iter: T) -> Self {
        let mut map: FxHashMap<_, GraphNode<'a>> = FxHashMap::default();

        for restriction in iter {
            {
                let container_node = map.entry(restriction.container).or_default();

                for (count, bag) in &restriction.content {
                    container_node.content.push((*count, bag));
                }
            }

            for (_count, bag) in &restriction.content {
                let node = map.entry(bag).or_default();

                node.containers.push(restriction.container);
            }
        }

        Self(map)
    }
}

impl<'a> RestrictionsGraph<'a> {
    pub(crate) fn count_possible_containers(&self, color: &str) -> usize {
        let mut lookup_stack = vec![color];
        let mut used_containers = FxHashSet::default();

        while let Some(container) = lookup_stack.pop() {
            used_containers.insert(container);

            for container in self
                .0
                .get(container)
                .into_iter()
                .flat_map(|node| &node.containers)
            {
                lookup_stack.push(container);
            }
        }

        // do not include self
        used_containers.len() - 1
    }

    pub(crate) fn count_children_bags(&self, color: &str) -> usize {
        let mut lookup_stack = vec![(1u32, color)];
        let mut count = 0usize;

        while let Some((bag_count, bag)) = lookup_stack.pop() {
            count += bag_count as usize;

            for content in self.0.get(bag).into_iter().flat_map(|node| &node.content) {
                lookup_stack.push((bag_count * content.0, content.1));
            }
        }

        // do not include self
        count - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_parser() {
        let restriction = BagsRestriction::try_from(
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        )
        .unwrap();

        assert_eq!(
            restriction,
            BagsRestriction {
                container: "vibrant plum",
                content: vec![(5, "faded blue"), (6, "dotted black")],
            }
        );

        let restriction =
            BagsRestriction::try_from("faded blue bags contain no other bags.").unwrap();

        assert_eq!(
            restriction,
            BagsRestriction {
                container: "faded blue",
                content: vec![],
            }
        );
    }

    #[test]
    fn test_example() {
        let restrictions = indoc!(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags."
        );

        let graph: RestrictionsGraph<'_> = restrictions
            .lines()
            .map(BagsRestriction::try_from)
            .map(Result::unwrap)
            .collect();

        assert_eq!(graph.count_possible_containers("shiny gold"), 4);
        assert_eq!(graph.count_children_bags("shiny gold"), 32);

        let restrictions = indoc!(
            "shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags."
        );

        let graph: RestrictionsGraph<'_> = restrictions
            .lines()
            .map(BagsRestriction::try_from)
            .map(Result::unwrap)
            .collect();

        assert_eq!(graph.count_children_bags("shiny gold"), 126);
    }
}
