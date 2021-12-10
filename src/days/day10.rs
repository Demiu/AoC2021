use nom::{
    bytes::complete::{is_a, tag},
    error::Error,
    multi::separated_list1,
};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

type ParserOutput = Vec<Line>;
type SolverInput = [Line];

pub enum Line {
    Incomplete(Vec<u8>),
    Corrupted(u32),
}

pub fn parse_input(file: &[u8]) -> anyhow::Result<ParserOutput> {
    fn parse_line(line: &[u8]) -> Line {
        let mut stack = Vec::new();
        for character in line {
            match character {
                b'(' | b'[' | b'{' | b'<' => stack.push(*character),
                b')' => {
                    if stack.pop().unwrap() != b'(' {
                        return Line::Corrupted(3);
                    }
                }
                b']' => {
                    if stack.pop().unwrap() != b'[' {
                        return Line::Corrupted(57);
                    }
                }
                b'}' => {
                    if stack.pop().unwrap() != b'{' {
                        return Line::Corrupted(1197);
                    }
                }
                b'>' => {
                    if stack.pop().unwrap() != b'<' {
                        return Line::Corrupted(25137);
                    }
                }
                _ => unreachable!(),
            }
        }
        Line::Incomplete(stack)
    }

    separated_list1::<_, _, _, Error<_>, _, _>(tag(b"\n"), is_a("([{<>}])"))(file)
        .map_err(|_| anyhow::anyhow!("Failed parsing braces"))
        .map(|t| t.1.into_par_iter().map(parse_line).collect::<Vec<_>>())
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    input
        .par_iter()
        .map(|line| {
            if let Line::Corrupted(err_val) = line {
                *err_val
            } else {
                0
            }
        })
        .sum()
}

pub fn solve_part2(input: &SolverInput) -> u64 {
    fn autocomplete_score(to_close: &[u8]) -> u64 {
        let mut score = 0;
        for bracket in to_close.iter().rev() {
            score *= 5;
            score += match *bracket {
                b'(' => 1,
                b'[' => 2,
                b'{' => 3,
                b'<' => 4,
                _ => unreachable!(),
            }
        }
        score
    }

    let mut scores: Vec<_> = input
        .par_iter()
        .filter_map(|line| {
            if let Line::Incomplete(to_close) = line {
                Some(autocomplete_score(to_close))
            } else {
                None
            }
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2] // scores are always odd in number
}
