
use anyhow::{Result, anyhow};
use nom::{
    bytes::complete::tag,
    multi::separated_list1,
    sequence::separated_pair,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{parse::parse_unsigned, traits::Intersect};

type ParserOutput = Vec<(u64, u64)>;
type SolverInput = [(u64, u64)];

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
    separated_list1(
        tag(b"\n"),
        separated_pair(parse_unsigned, tag(b","), parse_unsigned),
    )(file)
    .map_err(|_| anyhow!("Failed parsing cells"))
    .map(|t| t.1)
}

pub fn solve_part1(input: &SolverInput) -> u64 {
    let len = input.len();
    (0..len)
        .flat_map(|i| ((i + 1)..len).map(move |j| rectsize(input[i], input[j])))
        .max()
        .expect("Should have >=two elements")
}

pub fn solve_part2(input: &SolverInput) -> u64 {
    let len = input.len();
    let rects: Vec<_> = (0..len)
        .flat_map(|i| ((i + 1)..len).map(move |j| (rectsize(input[i], input[j]), i, j)))
        .collect();
    // tuples (is same x, value of the same dimension, range of the diff dimension)
    let connections: Vec<_> = [(len - 1, 0)]
        .into_iter()
        .chain((0..(len - 1)).map(|i| (i, i + 1)))
        .map(|(i, j)| {
            let (l, r) = (input[i], input[j]);
            match l.1 == r.1 {
                true => (false, l.1, l.0.min(r.0)..=l.0.max(r.0)),
                false => (true, l.0, l.1.min(r.1)..=l.1.max(r.1)),
            }
        })
        .collect();

    rects
        .par_iter()
        .filter_map(|&(size, i, j)| {
            let (l, r) = (input[i], input[j]);
            let x_inner = (l.0.min(r.0) + 1)..l.0.max(r.0);
            let y_inner = (l.1.min(r.1) + 1)..l.1.max(r.1);
            connections
                .iter()
                .find(|&t| match t {
                    (true, x, ys) => x_inner.contains(x) && ys.intersect_with(&y_inner).is_some(),
                    (false, y, xs) => y_inner.contains(y) && xs.intersect_with(&x_inner).is_some(),
                })
                .map_or(Some(size), |_| None)
        })
        .max()
        .expect("Should have at least one valid rect")
}

fn rectsize(l: (u64, u64), r: (u64, u64)) -> u64 {
    (l.0.abs_diff(r.0) + 1) * (l.1.abs_diff(r.1) + 1)
}

#[cfg(test)]
mod test {
    use concat_with::concat_line;

    use super::*;

    const EXAMPLE: &[u8] =
        concat_line!("7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3",).as_bytes();

    #[test]
    fn parse_example() {
        let parsed = rules::parse_expect!(EXAMPLE, "example");
        assert_eq!(
            parsed,
            [
                (7, 1),
                (11, 1),
                (11, 7),
                (9, 7),
                (9, 5),
                (2, 5),
                (2, 3),
                (7, 3),
            ]
        );
    }

    rules::make_test_for_day!(example, EXAMPLE, 50, 24);
}
