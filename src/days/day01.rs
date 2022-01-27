use anyhow::{anyhow, Result};
use nom::{bytes::complete::tag, multi::separated_list1};

use crate::parse::parse_unsigned;

type ParserOutput = Vec<u32>;
type SolverInput = [u32];

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
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
    solution
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
    solution
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = concat!(
        "199\n", "200\n", "208\n", "210\n", "200\n", "207\n", "240\n", "269\n", "260\n", "263\n",
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        let parsed = parse_input(EXAMPLE);
        assert!(parsed.is_ok(), "Failed parsing example input");
        assert_eq!(
            parsed.unwrap(),
            [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
        );
    }

    crate::macros::make_test_for_day!(example, EXAMPLE, 7, 5);
}
