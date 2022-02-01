use nom::{
    bytes::complete::{is_a, tag},
    error::Error,
    multi::separated_list1,
};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

type ParserOutput = Vec<Line>;
type SolverInput = [Line];

#[derive(Debug, PartialEq)]
pub enum Line {
    Incomplete(Vec<u8>),
    Corrupted(u32),
}

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

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = concat!(
        "[({(<(())[]>[[{[]{<()<>>\n",
        "[(()[<>])]({[<{<<[]>>(\n",
        "{([(<{}[<>[]}>{[]{[(<()>\n",
        "(((({<>}<{<{<>}{[]{[]{}\n",
        "[[<[([]))<([[{}[[()]]]\n",
        "[{[{({}]{}}([{[{{{}}([]\n",
        "{<[[]]>}<{[{[{[]{()[[[]\n",
        "[<(<(<(<{}))><([]([]()\n",
        "<{([([[(<>()){}]>(<<{{\n",
        "<{([{{}}[<[[[<>{}]]]>[]]\n",
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        let parsed = crate::macros::parse_expect!(EXAMPLE, "example");

        assert_eq!(parsed[2], Line::Corrupted(1197));
        assert_eq!(parsed[4], Line::Corrupted(3));
        assert_eq!(parsed[5], Line::Corrupted(57));
        assert_eq!(parsed[7], Line::Corrupted(3));
        assert_eq!(parsed[8], Line::Corrupted(25137));
        for i in 0..parsed.len() {
            if let Ok(_) = [2, 4, 5, 7, 8].binary_search(&i) {
                continue;
            }
            assert!(matches!(parsed[i], Line::Incomplete(_)));
        }
    }

    crate::macros::make_test_for_day!(example, EXAMPLE, 26397, 288957);

    #[test]
    fn part2_example_scores() {
        let parsed = crate::macros::parse_expect!(EXAMPLE, "example");

        let get_incomplete = |idx| match &parsed[idx] {
            Line::Incomplete(v) => v,
            _ => panic!("Trying to extract not from an incomplete line"),
        };

        assert_eq!(autocomplete_score(&get_incomplete(0)), 288957);
        assert_eq!(autocomplete_score(&get_incomplete(1)), 5566);
        assert_eq!(autocomplete_score(&get_incomplete(3)), 1480781);
        assert_eq!(autocomplete_score(&get_incomplete(6)), 995444);
        assert_eq!(autocomplete_score(&get_incomplete(9)), 294);
    }
}
