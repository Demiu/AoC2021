use std::collections::VecDeque;

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while},
    character::{complete::newline, is_digit, is_newline, is_space},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
};

use crate::parse::parse_unsigned;

// 0 - bottom of the stack
type Stack = VecDeque<u8>;
type ParserOutput = (Vec<Stack>, Vec<(usize, usize, usize)>);
type SolverInput = ParserOutput;

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
    let parse_crate = delimited(tag(b"["), take(1usize), tag(b"]"));
    let parse_crate_opt = map(alt((parse_crate, tag(b"   "))), |matched: &[u8]| {
        (matched.len() == 1).then_some(matched[0])
    });
    let parse_extra_inbetween = take_while(|c| is_space(c) || is_digit(c) || is_newline(c));
    let parse_stacks = map(
        separated_list1(newline, separated_list1(tag(b" "), parse_crate_opt)),
        |crates_vv| {
            let acc = vec![VecDeque::new(); crates_vv[0].len()];
            crates_vv.into_iter().fold(acc, |mut acc, crate_line| {
                crate_line.into_iter().enumerate().for_each(|(i, oc)| {
                    if let Some(c) = oc {
                        acc[i].push_front(c)
                    }
                });
                acc
            })
        },
    );
    let parse_instructions = map(
        separated_list1(
            tag(b"\n"),
            tuple((
                tag(b"move "),
                parse_unsigned::<usize>,
                tag(b" from "),
                parse_unsigned::<usize>,
                tag(b" to "),
                parse_unsigned::<usize>,
            )),
        ),
        |lines| lines.into_iter().map(|t| (t.1, t.3 - 1, t.5 - 1)).collect(),
    );
    separated_pair(parse_stacks, parse_extra_inbetween, parse_instructions)(file)
        .map_err(move |_| anyhow!("Parser failed"))
        .map(move |t| t.1)
}

pub fn solve_part1(input: &SolverInput) -> String {
    let mut stacks = input.0.clone();
    input.1.iter().for_each(|&(count, from, to)| {
        let pivot = stacks[from].len() - count;
        let to_move = stacks[from].split_off(pivot);
        stacks[to].extend(to_move.into_iter().rev());
    });
    stacks
        .into_iter()
        .flat_map(|mut v| v.pop_back().map(|c| c as char))
        .collect()
}

pub fn solve_part2(input: &SolverInput) -> String {
    let mut stacks = input.0.clone();
    input.1.iter().for_each(|&(count, from, to)| {
        let pivot = stacks[from].len() - count;
        let to_move = stacks[from].split_off(pivot);
        stacks[to].extend(to_move.into_iter());
    });
    stacks
        .into_iter()
        .flat_map(|mut v| v.pop_back().map(|c| c as char))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = concat!(
        "    [D]    \n",
        "[N] [C]    \n",
        "[Z] [M] [P]\n",
        " 1   2   3 \n",
        "\n",
        "move 1 from 2 to 1\n",
        "move 3 from 1 to 3\n",
        "move 2 from 2 to 1\n",
        "move 1 from 1 to 2\n",
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        let parsed = crate::macros::parse_expect!(EXAMPLE, "example");
        let desired_stacks = vec![
            vec![b'Z', b'N'].into(),
            vec![b'M', b'C', b'D'].into(),
            vec![b'P'].into(),
        ];
        let desired_instructions = vec![(1, 1, 0), (3, 0, 2), (2, 1, 0), (1, 0, 1)];
        assert_eq!(parsed, (desired_stacks, desired_instructions,));
    }

    crate::macros::make_test_for_day!(example, EXAMPLE, "CMZ", "MCD");
}
