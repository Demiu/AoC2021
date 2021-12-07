use anyhow::{anyhow, Result};
use itertools::sorted;
use nom::{bytes::complete::tag, multi::separated_list1};

use crate::parse::parse_unsigned;

type SolverInput = Vec<u32>;

// u32::abs_diff is nightly :(
fn abs_diff(l: u32, r: u32) -> u32 {
    if l > r {
        l - r
    } else {
        r - l
    }
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    Ok(sorted(
        separated_list1(tag(b","), parse_unsigned)(file)
            .map_err(|_| anyhow!("Failed parsing list of crab positions"))?
            .1,
    )
    .collect())
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let median = if input.len() % 2 == 1 {
        input[(input.len() / 2) + 1]
    } else {
        let left = input[(input.len() / 2) - 1];
        let right = input[(input.len() / 2)];
        (right + left) / 2
    };
    input.iter().map(|p| abs_diff(*p, median)).sum()
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let average = {
        let sum: u32 = input.iter().sum();
        let n = input.len() as u32;
        sum / n
    };

    let mut total_sum = 0;
    for pos in input {
        // S = ((a0 + an)*n) / 2
        let difference = abs_diff(*pos, average);
        let n = difference + 1;
        let sum = (difference * n) / 2;
        total_sum += sum;
    }
    total_sum
}
