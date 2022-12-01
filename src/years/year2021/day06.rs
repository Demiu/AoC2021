use anyhow::{anyhow, Result};
use nom::{bytes::complete::tag, multi::separated_list1};

use crate::parse::parse_unsigned;

type SolverInput = [u32; TOTAL_CATEGORIES];

const CYCLE_LENGTH: usize = 7;
const NEW_CYCLE_EXTRA: usize = 2;
const TOTAL_CATEGORIES: usize = CYCLE_LENGTH + NEW_CYCLE_EXTRA;

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    let numbers = separated_list1(tag(b","), parse_unsigned::<usize>)(file)
        .map_err(|_| anyhow!("Failed parsing lines"))?
        .1;
    let mut lanternfish = [0; TOTAL_CATEGORIES];
    for fish in numbers {
        lanternfish[fish] += 1;
    }
    Ok(lanternfish)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    const DAYS: usize = 80;
    let mut lanternfish = *input;
    for _ in 0..DAYS {
        let expired = lanternfish[0];
        for i in 0..8 {
            lanternfish[i] = lanternfish[i + 1];
        }
        lanternfish[6] += expired;
        lanternfish[8] = expired;
    }
    lanternfish.iter().sum()
}

pub fn solve_part2(input: &SolverInput) -> u64 {
    const DAYS: usize = 256;
    let mut lanternfish = input.map(|v| v as u64);
    for _ in 0..DAYS {
        let expired = lanternfish[0];
        for i in 0..8 {
            lanternfish[i] = lanternfish[i + 1];
        }
        lanternfish[6] += expired;
        lanternfish[8] = expired;
    }
    lanternfish.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = "3,4,3,1,2".as_bytes();

    #[test]
    fn parse_example() {
        let parsed = crate::macros::parse_expect!(EXAMPLE, "example");
        assert_eq!(parsed, [0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }

    crate::macros::make_test_for_day!(example, EXAMPLE, 5934, 26984457539);
}
