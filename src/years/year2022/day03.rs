use std::collections::{hash_map::RandomState, HashSet};

use anyhow::Result;

type ParserOutput<'a> = Vec<&'a [u8]>;
type SolverInput<'a> = [&'a [u8]];

fn ascii_to_priority(c: u8) -> Option<i32> {
    match c {
        b'a'..=b'z' => Some((c - b'a' + 1) as i32),
        b'A'..=b'Z' => Some((c - b'A' + 27) as i32),
        _ => None,
    }
}

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
    Ok(file
        .split(|c| *c == b'\n')
        .filter(|line| line.len() > 0)
        .collect())
}

pub fn solve_part1(input: &SolverInput) -> i32 {
    input
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(l, r)| {
            let l = HashSet::<_, RandomState>::from_iter(l.iter().copied());
            let r = HashSet::<_, RandomState>::from_iter(r.iter().copied());
            l.intersection(&r)
                .map(|&c| ascii_to_priority(c))
                .sum::<Option<i32>>()
                .unwrap_or(i32::MIN)
        })
        .sum()
}

pub fn solve_part2(input: &SolverInput) -> i32 {
    input
        .chunks(3)
        .map(|chunk| {
            chunk
                .into_iter()
                .map(|&line| HashSet::<_, RandomState>::from_iter(line.iter().copied()))
                .reduce(|hs1, hs2| hs1.intersection(&hs2).copied().collect())
                .and_then(|hs| hs.iter().map(|&c| ascii_to_priority(c)).sum())
                .unwrap_or(i32::MIN)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = concat!(
        "vJrwpWtwJgWrhcsFMMfFFhFp\n",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
        "PmmdzqPrVvPwwTWBwg\n",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
        "ttgJtRGJQctTZtZT\n",
        "CrZsJsPPZsGzwwsLwLmpwMDw\n",
    )
    .as_bytes();

    crate::macros::make_test_for_day!(example, EXAMPLE, 157, 70);
}
