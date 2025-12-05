use std::{collections::BinaryHeap, iter::FromIterator};

use anyhow::{Result, anyhow};
use nom::{bytes::complete::tag, multi::separated_list1};

use crate::parse::parse_unsigned;

type ParserOutput = Vec<u32>;
type SolverInput = [u32];

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
    let parse_elf = |entries| {
        separated_list1(tag(b"\n"), parse_unsigned)(entries)
            .map(|(rest, calories)| (rest, calories.iter().sum()))
    };
    separated_list1(tag("\n\n"), parse_elf)(file)
        .map_err(move |_| anyhow!("Parser failed"))
        .map(move |t| t.1)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    *input
        .iter()
        .max()
        .expect("At least one calorie entry is required")
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let mut heap = BinaryHeap::from_iter(input.iter().cloned());
    [heap.pop(), heap.pop(), heap.pop()]
        .into_iter()
        .flatten()
        .take(3)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = concat!(
        "1000\n", "2000\n", "3000\n", "\n", "4000\n", "\n", "5000\n", "6000\n", "\n", "7000\n",
        "8000\n", "9000\n", "\n", "10000\n",
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        let parsed = rules::parse_expect!(EXAMPLE, "example");
        assert_eq!(parsed, [6000, 4000, 11000, 24000, 10000]);
    }

    rules::make_test_for_day!(example, EXAMPLE, 24000, 45000);
}
