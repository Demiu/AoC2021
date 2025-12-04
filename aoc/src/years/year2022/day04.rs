use std::ops::RangeInclusive;

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair,
};

use crate::parse::parse_range_unsigned;

type ElfPair = (RangeInclusive<u8>, RangeInclusive<u8>);
type ParserOutput = Vec<ElfPair>;
type SolverInput = [ElfPair];

fn ranges_full_overlap(l: &RangeInclusive<u8>, r: &RangeInclusive<u8>) -> bool {
    match (
        l.start() > r.start(),
        l.start() == r.start(),
        l.end() < r.end(),
        l.end() == r.end(),
    ) {
        (true, false, false, false) => false,
        (false, false, true, false) => false,
        _ => true,
    }
}

fn ranges_overlap(l: &RangeInclusive<u8>, r: &RangeInclusive<u8>) -> bool {
    l.start() <= r.end() && r.start() <= l.end()
}

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
    separated_list1(
        newline,
        separated_pair(
            parse_range_unsigned(tag(b"-")),
            tag(b","),
            parse_range_unsigned(tag(b"-")),
        ),
    )(file)
    .map_err(move |_| anyhow!("Parser failed"))
    .map(move |t| t.1)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    input
        .iter()
        .map(|t| ranges_full_overlap(&t.0, &t.1) as u32)
        .sum()
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    input
        .iter()
        .map(|t| ranges_overlap(&t.0, &t.1) as u32)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = concat!(
        "2-4,6-8\n",
        "2-3,4-5\n",
        "5-7,7-9\n",
        "2-8,3-7\n",
        "6-6,4-6\n",
        "2-6,4-8\n",
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        let parsed = rules::parse_expect!(EXAMPLE, "example");
        assert_eq!(
            parsed,
            [
                (2..=4, 6..=8),
                (2..=3, 4..=5),
                (5..=7, 7..=9),
                (2..=8, 3..=7),
                (6..=6, 4..=6),
                (2..=6, 4..=8),
            ]
        );
    }

    rules::make_test_for_day!(example, EXAMPLE, 2, 4);
}
