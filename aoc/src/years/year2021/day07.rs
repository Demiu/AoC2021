use anyhow::{Result, anyhow};
use nom::{bytes::complete::tag, multi::separated_list1};

use crate::parse::parse_unsigned;

type ParserOutput = Vec<u32>;
type SolverInput = [u32];

// u32::abs_diff is nightly :(
fn abs_diff(l: u32, r: u32) -> u32 {
    l.abs_diff(r)
}

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
    let mut numbers = separated_list1(tag(b","), parse_unsigned)(file)
        .map_err(|_| anyhow!("Failed parsing list of crab positions"))?
        .1;
    numbers.sort_unstable();
    Ok(numbers)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let median = if input.len() % 2 == 1 {
        input[(input.len() / 2) + 1]
    } else {
        let left = input[(input.len() / 2) - 1];
        let right = input[input.len() / 2];
        (right + left) / 2
    };
    input.iter().map(|p| abs_diff(*p, median)).sum()
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let fuel_sum = |to| {
        let mut total_sum = 0;
        for pos in input {
            // S = ((a0 + an)*n) / 2
            let difference = abs_diff(*pos, to);
            let n = difference + 1;
            let sum = (difference * n) / 2;
            total_sum += sum;
        }
        total_sum
    };

    let average = {
        let sum: u32 = input.iter().sum();
        let n = input.len() as u32;
        sum / n
    };

    u32::min(fuel_sum(average), fuel_sum(average + 1))
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = "16,1,2,0,4,2,7,1,2,14".as_bytes();

    #[test]
    fn parse_example() {
        let parsed = rules::parse_expect!(EXAMPLE, "example");
        let mut desired = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        desired.sort_unstable();
        assert_eq!(parsed, desired);
    }

    rules::make_test_for_day!(example, EXAMPLE, 37, 168);
}
