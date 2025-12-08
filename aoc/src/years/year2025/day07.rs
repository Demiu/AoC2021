use anyhow::{Result, anyhow};
use nom::{
    Err,
    bytes::complete::{is_a, tag},
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
};

type SolverInput<'a> = (usize, usize, Vec<&'a [u8]>);

pub fn parse_input(file: &[u8]) -> Result<SolverInput<'_>> {
    const DOT_ARR: [u8; 1] = [b'.'];
    separated_pair(
        tuple((is_a(DOT_ARR), tag(b"S"), is_a(DOT_ARR))),
        tag(b"\n"),
        separated_list1(tag(b"\n"), is_a([b'.', b'^'])),
    )(file)
    .map_err(|_: Err<Error<_>>| anyhow!("Failed parsing cells"))
    .map(|t| t.1)
    .map(|(fst, rst)| (fst.0.len(), fst.0.len() + fst.2.len() + 1, rst))
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut state = vec![false; input.1];
    let mut new = vec![false; input.1];
    state[input.0] = true;
    input
        .2
        .iter()
        .map(|row| {
            new.iter_mut().for_each(|b| *b = false);
            let splits: u32 = row
                .iter()
                .copied()
                .zip(state.iter().copied())
                .enumerate()
                .map(|(i, t)| {
                    match t {
                        (b'.', true) => {
                            new[i] = true;
                            0
                        }
                        (b'^', true) => {
                            // This can under/overflow, manually checked the input for no ^s at the edge
                            new[i + 1] = true;
                            new[i - 1] = true;
                            1
                        }
                        _ => 0,
                    }
                })
                .sum();
            std::mem::swap(&mut state, &mut new);
            splits
        })
        .sum()
}

pub fn solve_part2(input: &SolverInput) -> u64 {
    let mut state = vec![0; input.1];
    let mut new = vec![0; input.1];
    state[input.0] = 1;
    input.2.iter().for_each(|row| {
        new.iter_mut().for_each(|c| *c = 0);
        row.iter()
            .copied()
            .zip(state.iter().copied())
            .enumerate()
            .for_each(|(i, t)| {
                match t {
                    (b'.', c) => new[i] += c,
                    (b'^', c) => {
                        // This can under/overflow, manually checked the input for no ^s at the edge
                        new[i + 1] += c;
                        new[i - 1] += c;
                    }
                    _ => (),
                }
            });
        std::mem::swap(&mut state, &mut new);
    });
    state.iter().sum::<u64>()
}

#[cfg(test)]
mod test {
    use concat_with::concat_line;

    use super::*;

    const EXAMPLE: &[u8] = concat_line!(
        ".......S.......",
        "...............",
        ".......^.......",
        "...............",
        "......^.^......",
        "...............",
        ".....^.^.^.....",
        "...............",
        "....^.^...^....",
        "...............",
        "...^.^...^.^...",
        "...............",
        "..^...^.....^..",
        "...............",
        ".^.^.^.^.^...^.",
        "...............",
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        let parsed = rules::parse_expect!(EXAMPLE, "example");
        assert_eq!(
            parsed,
            (
                7,
                15,
                [
                    &b"..............."[..],
                    &b".......^......."[..],
                    &b"..............."[..],
                    &b"......^.^......"[..],
                    &b"..............."[..],
                    &b".....^.^.^....."[..],
                    &b"..............."[..],
                    &b"....^.^...^...."[..],
                    &b"..............."[..],
                    &b"...^.^...^.^..."[..],
                    &b"..............."[..],
                    &b"..^...^.....^.."[..],
                    &b"..............."[..],
                    &b".^.^.^.^.^...^."[..],
                    &b"..............."[..],
                ]
                .into()
            )
        );
    }

    rules::make_test_for_day!(example, EXAMPLE, 21, 40);
}
