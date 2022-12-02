use std::io::BufRead;

use anyhow::{anyhow, Result};

type ParserOutput = Vec<(u8, u8)>;
type SolverInput = [(u8, u8)];

// Rock/Lose is A/X
// Paper/Draw is B/Y
// Scissors/Win is C/Z

const SCORE_TABLE: [[(u32, u32); 3]; 3] = {
    let mut ret = [[(0, 0); 3]; 3];
    let mut them = 0;
    while them < 3 {
        let mut me = 0;
        while me < 3 {
            ret[them][me].0 = (me + 1 + ((4 + me - them) % 3 * 3)) as u32;
            ret[them][me].1 = (me * 3 + ((2 + me + them) % 3) + 1) as u32;
            me += 1;
        }
        them += 1;
    }
    ret
};

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
    Ok(file
        .split(|c| *c == b'\n')
        .filter_map(|line| match line {
            &[l, _, r, ..] => Some((l - b'A', r - b'X')),
            _ => None,
        })
        .collect())
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    input
        .iter()
        .map(|t| SCORE_TABLE[t.0 as usize][t.1 as usize].0)
        .sum()
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    input
        .iter()
        .map(|t| SCORE_TABLE[t.0 as usize][t.1 as usize].1)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = concat!("A Y\n", "B X\n", "C Z\n",).as_bytes();

    #[test]
    fn parse_example() {
        let parsed = crate::macros::parse_expect!(EXAMPLE, "example");
        assert_eq!(parsed, [(0, 1), (1, 0), (2, 2)]);
    }

    crate::macros::make_test_for_day!(example, EXAMPLE, 15, 12);
}
