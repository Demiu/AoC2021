use anyhow::{Result, anyhow};
use nom::{
    bytes::complete::tag, character::complete::digit1, error::Error, multi::separated_list1,
};

type ParserOutput<'a> = Vec<&'a [u8]>;
type SolverInput<'a> = [&'a [u8]];

pub fn parse_input(file: &[u8]) -> Result<ParserOutput<'_>> {
    separated_list1(tag("\n"), digit1)(file)
        .map_err(move |_: nom::Err<Error<_>>| anyhow!("Parser failed"))
        .map(move |t| t.1)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    input
        .iter()
        .map(|line| {
            let (idx, tens) = line[..line.len() - 1]
                .iter()
                .enumerate()
                .reduce(|best, new| if new.1 > best.1 { new } else { best })
                .expect("Battery bank cannot be shorter than 2");
            let ones = line[idx + 1..]
                .iter()
                .max()
                .expect("There will always be a free last digit for ones");
            (10 * (tens - b'0') + (ones - b'0')) as u32
        })
        .sum()
}

pub fn solve_part2(input: &SolverInput) -> u64 {
    let make_best_finder = |n| {
        move |segment: &[u8]| {
            segment[..segment.len() - n]
                .iter()
                .copied()
                .enumerate()
                .reduce(|best, new| if new.1 > best.1 { new } else { best })
                .expect("Remaining battery bank size should never be less than 1")
        }
    };
    input
        .iter()
        .map(|&line| {
            (0..12)
                .rev()
                .fold((0, line), |(mut out, seg), nth| {
                    let (idx, val) = make_best_finder(nth)(seg);
                    out *= 10;
                    out += (val - b'0') as u64;
                    (out, &seg[idx + 1..])
                })
                .0
        })
        .sum()
}

#[cfg(test)]
mod test {
    use concat_with::concat_line;

    use super::*;

    const EXAMPLE: &[u8] = concat_line!(
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111",
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        let parsed = rules::parse_expect!(EXAMPLE, "example");
        assert_eq!(
            parsed,
            [
                b"987654321111111",
                b"811111111111119",
                b"234234234234278",
                b"818181911112111",
            ]
        );
    }

    rules::make_test_for_day!(example, EXAMPLE, 357, 3121910778619);
}
