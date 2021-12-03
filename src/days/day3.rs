use anyhow::{anyhow, Result};
use nom::{bytes::complete::tag, character::complete::digit1, multi::separated_list1};

use crate::parse::parse_u32_radix;

pub struct SolverInput<'a> {
    lines: Vec<(&'a [u8], u32)>,
    line_length: usize,
}

pub fn parse_input<'a>(file: &'a [u8]) -> Result<SolverInput<'a>> {
    let line_length = file
        .iter()
        .copied()
        .take_while(move |c| *c == b'1' || *c == b'0')
        .count();
    separated_list1(tag("\n"), digit1::<_, nom::error::Error<_>>)(file)
        .map_err(move |_| anyhow!("Line parser failed"))
        .map(move |t| {
            let mut lines: Vec<(&'a [u8], u32)> = t.1.into_iter().map(|line| 
                (line, parse_u32_radix::<'a, nom::error::Error<_>>(2)(line).unwrap().1)
            ).collect();
            lines.sort_unstable_by_key(|line_tuple| line_tuple.1);
            SolverInput {
                lines,
                line_length,
            }
        })
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut counts = vec![0; input.line_length];
    for line in input.lines.iter().copied() {
        for (i, ch) in line.0.iter().enumerate() {
            match ch {
                b'1' => counts[i] += 1,
                b'0' => (),
                _ => unreachable!(),
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for count in counts {
        gamma *= 2;
        epsilon *= 2;
        if count * 2 > input.lines.len() {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    return gamma * epsilon;
}
