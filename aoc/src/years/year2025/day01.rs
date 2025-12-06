use anyhow::{Result, anyhow};
use nom::{
    bytes::complete::tag, character::complete::anychar, combinator::map_opt,
    multi::separated_list1, sequence::pair,
};

use crate::parse::parse_unsigned;

type Rotation = (bool, i32);
type ParserOutput = Vec<Rotation>;
type SolverInput = [Rotation];

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
    let map_lr = |i| match i {
        'L' => Some(false),
        'R' => Some(true),
        _ => None,
    };
    separated_list1(tag("\n"), pair(map_opt(anychar, map_lr), parse_unsigned))(file)
        .map_err(move |_| anyhow!("Parser failed"))
        .map(move |t| t.1)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    input
        .iter()
        .fold((50i32, 0), |(acc, cnt), rot| {
            let new = match *rot {
                (true, deg) => acc + deg,
                (false, deg) => acc - deg,
            };

            (new, cnt + (new % 100 == 0) as u32)
        })
        .1
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    input
        .iter()
        .fold((50i32, 0), |(acc, cnt), &(dir, deg)| {
            let after = match dir {
                true => acc + deg,
                false => acc - deg,
            };
            let mut cycles = after / 100;
            let mut new = after + (cycles * -100);
            cycles = cycles.abs();
            if new < 0 {
                new += 100;
                if acc != 0 {
                    cycles += 1;
                }
            } else if !dir && new == 0 {
                cycles += 1;
            }
            (new, cnt + cycles as u32)
        })
        .1
}

#[cfg(test)]
mod test {
    use concat_with::concat_line;

    use super::*;

    const EXAMPLE: &[u8] = concat_line!(
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        let parsed = rules::parse_expect!(EXAMPLE, "example");
        assert_eq!(
            parsed,
            [
                (false, 68),
                (false, 30),
                (true, 48),
                (false, 5),
                (true, 60),
                (false, 55),
                (false, 1),
                (false, 99),
                (true, 14),
                (false, 82),
            ]
        );
    }

    rules::make_test_for_day!(example, EXAMPLE, 3, 6);
}
