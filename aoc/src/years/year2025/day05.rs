use std::ops::RangeInclusive;

use anyhow::{Result, anyhow};
use nom::{
    bytes::complete::tag, combinator::map, multi::separated_list1, sequence::separated_pair,
};

use crate::{parse::parse_unsigned, traits::Union};

type SolverInput = (Vec<RangeInclusive<u64>>, Vec<u64>);

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    separated_pair(
        separated_list1(
            tag(b"\n"),
            map(
                separated_pair(parse_unsigned, tag(b"-"), parse_unsigned),
                |(l, h)| l..=h,
            ),
        ),
        tag(b"\n\n"),
        separated_list1(tag(b"\n"), parse_unsigned),
    )(file)
    .map_err(|_| anyhow!("Failed parsing cells"))
    .map(|t| t.1)
}

pub fn solve_part1(input: &SolverInput) -> usize {
    let (ranges, ingredients) = (&input.0[..], &input.1[..]);
    ingredients
        .iter()
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count()
}

pub fn solve_part2(input: &SolverInput) -> u64 {
    type Rri<'a> = &'a RangeInclusive<u64>;
    let mut ranges = Vec::new();
    for new in input.0.iter().cloned() {
        let mut to_insert = new;
        loop {
            let candidate = ranges.partition_point(|r: Rri| r.end() < to_insert.start());
            if candidate == ranges.len() {
                ranges.push(to_insert);
                break;
            }
            match ranges[candidate].union_with(&to_insert) {
                Some(combined) => {
                    to_insert = combined;
                    ranges.remove(candidate);
                }
                None => {
                    ranges.insert(candidate, to_insert);
                    break;
                }
            }
        }
    }
    ranges
        .into_iter()
        .map(RangeInclusive::into_inner)
        .map(|(l, h)| h - l + 1)
        .sum()
}

#[cfg(test)]
mod test {
    use concat_with::concat_line;

    use super::*;

    const EXAMPLE: &[u8] = concat_line!(
        "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        let parsed = rules::parse_expect!(EXAMPLE, "example");
        assert_eq!(
            parsed,
            (
                [3..=5, 10..=14, 16..=20, 12..=18,].into(),
                [1, 5, 8, 11, 17, 32].into()
            )
        );
    }

    rules::make_test_for_day!(example, EXAMPLE, 3, 14);
}
