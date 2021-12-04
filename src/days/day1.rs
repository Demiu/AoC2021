use anyhow::{anyhow, Result};
use nom::{bytes::complete::tag, multi::separated_list1};

use crate::parse::parse_unsigned;

type SolverInput = Vec<u64>;

pub fn parse_input<'a>(file: &'a [u8]) -> Result<SolverInput> {
    separated_list1(tag("\n"), parse_unsigned)(file)
        .map_err(move |_| anyhow!("Parser failed"))
        .map(move |t| t.1)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut solution = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            solution += 1;
        }
    }
    return solution;
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let mut solution = 0;
    for i in 3..input.len() {
        // n[i-2] + n[i-1] + n[i] > n[i-3] + n[i-2] + n[i-1]
        // n[i] > n[i-3]
        if input[i] > input[i - 3] {
            solution += 1;
        }
    }
    return solution;
}
