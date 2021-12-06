use anyhow::{anyhow, Result};
use nom::{multi::separated_list1, bytes::complete::tag};

use crate::parse::parse_unsigned;

type SolverInput = Vec<u32>;

// todo make the 9 const

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    let numbers = separated_list1(tag(b","), parse_unsigned::<usize>)(file)
        .map_err(|_| anyhow!("Failed parsing lines"))?.1;
    let mut lanternfish = vec![0; 9];
    for fish in numbers {
        lanternfish[fish] += 1;
    }
    Ok(lanternfish)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut lanternfish = input.clone();
    for _ in 0..80 {
        let expired = lanternfish[0];
        for i in 0..8 {
            lanternfish[i] = lanternfish[i+1];
        }
        lanternfish[6] += expired;
        lanternfish[8] = expired;
    }
    lanternfish.iter().sum()
}
