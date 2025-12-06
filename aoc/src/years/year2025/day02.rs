use std::{
    collections::{BTreeSet, VecDeque},
    iter::repeat,
};

use anyhow::{Result, anyhow};
use nom::{
    Err, bytes::complete::tag, character::complete::digit1, combinator::map_opt, error::Error,
    multi::separated_list1, sequence::separated_pair,
};

use crate::parse::parse_unsigned_radix;

type Range<'a> = (&'a [u8], &'a [u8], u64, u64);
type ParserOutput<'a> = Vec<Range<'a>>;
type SolverInput<'a> = [Range<'a>];

pub fn parse_input(file: &[u8]) -> Result<ParserOutput<'_>> {
    let map_range = |(l, r)| {
        Some((
            l,
            r,
            parse_unsigned_radix(l, 10)?,
            parse_unsigned_radix(r, 10)?,
        ))
    };
    separated_list1(
        tag(b","),
        map_opt(separated_pair(digit1, tag(b"-"), digit1), map_range),
    )(file)
    .map_err(move |_: Err<Error<_>>| anyhow!("Parser failed"))
    .map(move |t| t.1)
}

pub fn solve_part1(input: &SolverInput) -> u64 {
    let mut invalid_sum = 0;
    for &(lo_str, _, low, high) in input {
        let mut repeated: VecDeque<_> = lo_str[..(lo_str.len() / 2)].iter().copied().collect();
        let mut parsed = parse_intstr_vd(&repeated, 2);
        while (..low).contains(&parsed) {
            increment_intstr(&mut repeated);
            parsed = parse_intstr_vd(&repeated, 2);
        }
        loop {
            if !(..=high).contains(&parsed) {
                break;
            }
            invalid_sum += parsed;
            increment_intstr(&mut repeated);
            parsed = parse_intstr_vd(&repeated, 2);
        }
    }
    invalid_sum
}

pub fn solve_part2(input: &SolverInput) -> u64 {
    let mut invalids = BTreeSet::new();
    for &(_, hi_str, low, high) in input {
        let maxlen = hi_str.len();
        for len in 1..=maxlen {
            let maxrep = hi_str.len() / len;
            for repeats in 2..=maxrep {
                if parse_intstr(b"9", 1, len * repeats) < low {
                    continue;
                }
                let mut snippet: VecDeque<_> =
                    [b'1'].into_iter().chain(repeat(b'0')).take(len).collect();
                let mut parsed = parse_intstr_vd(&snippet, repeats);
                while parsed < low {
                    increment_intstr(&mut snippet);
                    parsed = parse_intstr_vd(&snippet, repeats);
                }
                loop {
                    if parsed > high {
                        break;
                    }
                    invalids.insert(parsed);
                    increment_intstr(&mut snippet);
                    parsed = parse_intstr_vd(&snippet, repeats);
                }
            }
        }
    }
    invalids.into_iter().sum()
}

fn parse_intstr_vd(v: &VecDeque<u8>, times: usize) -> u64 {
    parse_intstr(v, v.len(), times)
}

fn parse_intstr<'a, I>(v: I, len: usize, times: usize) -> u64
where
    I: IntoIterator<Item = &'a u8>,
    I::IntoIter: Clone,
{
    parse_unsigned_radix(v.into_iter().cycle().take(len * times), 10)
        .expect("exclusively ASCII digit bytes chained together")
}

fn increment_intstr(v: &mut VecDeque<u8>) {
    match v.iter().rposition(|&b| b < b'9') {
        Some(p) => {
            v[p] += 1;
            v.iter_mut().skip(p + 1).for_each(|c| *c = b'0');
        }
        None => {
            v.iter_mut().for_each(|c| *c = b'0');
            v.push_front(b'1');
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = concat!(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,",
        "1698522-1698528,446443-446449,38593856-38593862,565653-565659,",
        "824824821-824824827,2121212118-2121212124"
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        let parsed = rules::parse_expect!(EXAMPLE, "example");
        assert_eq!(
            parsed,
            [
                (&b"11"[..], &b"22"[..], 11, 22),
                (&b"95"[..], &b"115"[..], 95, 115),
                (&b"998"[..], &b"1012"[..], 998, 1012),
                (
                    &b"1188511880"[..],
                    &b"1188511890"[..],
                    1188511880,
                    1188511890
                ),
                (&b"222220"[..], &b"222224"[..], 222220, 222224),
                (&b"1698522"[..], &b"1698528"[..], 1698522, 1698528),
                (&b"446443"[..], &b"446449"[..], 446443, 446449),
                (&b"38593856"[..], &b"38593862"[..], 38593856, 38593862),
                (&b"565653"[..], &b"565659"[..], 565653, 565659),
                (&b"824824821"[..], &b"824824827"[..], 824824821, 824824827),
                (
                    &b"2121212118"[..],
                    &b"2121212124"[..],
                    2121212118,
                    2121212124
                ),
            ]
        );
    }

    rules::make_test_for_day!(example, EXAMPLE, 1227775554, 4174379265);
}
