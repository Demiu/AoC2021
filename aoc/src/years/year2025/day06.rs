use anyhow::{Result, anyhow};
use nom::{
    Err,
    bytes::complete::{is_a, is_not, tag},
    character::complete::space1,
    combinator::recognize,
    error::Error,
    multi::{many1, separated_list1},
    sequence::{pair, separated_pair},
};

use crate::parse::parse_unsigned_radix;

type ParserOutput<'a> = Vec<(u8, Vec<&'a [u8]>)>;
type SolverInput<'a> = [(u8, Vec<&'a [u8]>)];

const ADD_SIGN: u8 = b'+';
const MUL_SIGN: u8 = b'*';

pub fn parse_input(file: &[u8]) -> Result<ParserOutput<'_>> {
    let (rows_raw, ops) = separated_pair(
        separated_list1(tag(b"\n"), is_not([ADD_SIGN, MUL_SIGN, b'\n'])),
        tag(b"\n"),
        many1(recognize(pair(is_a([ADD_SIGN, MUL_SIGN]), space1))),
    )(file)
    .inspect_err(|e| panic!("{e}"))
    .map_err(|_: Err<Error<_>>| anyhow!("Failed parsing cells"))?
    .1;

    let mut xoff = 0;
    let mut out = Vec::with_capacity(ops.len());
    for op in ops.into_iter() {
        let len = op.len();
        let mut nums = Vec::with_capacity(rows_raw.len());
        for row in &rows_raw {
            nums.push(&row[xoff..(xoff + len)]);
        }
        let sign = *op.first()
            .expect("At least one element guaranteed by parsers");
        out.push((sign, nums));
        xoff += len;
    }

    Ok(out)
}

pub fn solve_part1(input: &SolverInput) -> u64 {
    input
        .iter()
        .map(|(op, col)| {
            let num_it = col.iter().map(|&arr| {
                parse_unsigned_radix::<_, u64>(arr.iter().filter(|&&c| c != b' '), 10)
            });
            match *op {
                ADD_SIGN => num_it.sum::<Option<u64>>(),
                MUL_SIGN => num_it.product(),
                _ => {
                    unreachable!("Guaranteed by parser");
                }
            }
        })
        .sum::<Option<u64>>()
        .expect("All arrays should be parsable")
}

pub fn solve_part2(input: &SolverInput) -> u64 {
    input
        .iter()
        .map(|(op, col)| {
            let nums = col.len();
            let num_it = (0..nums)
                .map(|n| {
                    parse_unsigned_radix::<_, u64>(
                        col.iter()
                            .filter_map(|arr| arr.get(n).filter(|&&c| c != b' ')),
                        10,
                    )
                })
                .filter(|o| o.is_some_and(|v| v != 0));
            match *op {
                ADD_SIGN => num_it.sum::<Option<u64>>(),
                MUL_SIGN => num_it.product(),
                _ => {
                    unreachable!("Guaranteed by parser");
                }
            }
        })
        .sum::<Option<u64>>()
        .expect("All arrays should be parsable")
}

#[cfg(test)]
mod test {
    use concat_with::concat_line;

    use super::*;

    const EXAMPLE: &[u8] = concat_line!(
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  ",
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        let parsed = rules::parse_expect!(EXAMPLE, "example");
        assert_eq!(
            parsed,
            [
                (MUL_SIGN, [&b"123 "[..], &b" 45 "[..], &b"  6 "[..]].into()),
                (ADD_SIGN, [&b"328 "[..], &b"64  "[..], &b"98  "[..]].into()),
                (MUL_SIGN, [&b" 51 "[..], &b"387 "[..], &b"215 "[..]].into()),
                (ADD_SIGN, [&b"64 "[..], &b"23 "[..], &b"314"[..]].into()),
            ]
        );
    }

    rules::make_test_for_day!(example, EXAMPLE, 4277556, 3263827);
}
