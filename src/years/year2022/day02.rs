use anyhow::{anyhow, Result};

type ParserOutput = Vec<(Rps, Rps)>;
type SolverInput = [(Rps, Rps)];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Rps {
    // A X; or lose
    Rock,
    // B Y; or draw
    Paper,
    // C Z; or win
    Scissors,
}

fn pick_score(pick: Rps) -> u32 {
    match pick {
        Rps::Rock => 1,
        Rps::Paper => 2,
        Rps::Scissors => 3,
    }
}

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
    let parse_rps = |symbol| match symbol {
        b'A' | b'X' => Ok(Rps::Rock),
        b'B' | b'Y' => Ok(Rps::Paper),
        b'C' | b'Z' => Ok(Rps::Scissors),
        _ => Err(anyhow!("Unsupported character in rps sequence")),
    };
    file
        .split(|c| *c == b'\n')
        .filter(|l| l.len() > 0)
        .map(|l| parse_rps(l[0]).ok().zip(parse_rps(l[2]).ok()).ok_or_else(|| anyhow!("Couldn't parse a line of input")))
        .collect()
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let win_score = |t| match t {
        (Rps::Rock, Rps::Paper) | (Rps::Paper, Rps::Scissors) | (Rps::Scissors, Rps::Rock) => 6,
        (x, y) if x == y => 3,
        // else we lost
        _ => 0,
    };
    input.iter().map(|t| pick_score(t.1) + win_score(*t)).sum()
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    // Alternative solution: treat picks as numbers: R=0, P=1, S=2
    // Then pick = (t.0 + t.1) % 3
    let strat_to_win_score = |s| match s {
        Rps::Rock => 0,
        Rps::Paper => 3,
        Rps::Scissors => 6,
    };
    let case_to_pick = |t| match t {
        (Rps::Paper, Rps::Rock) | (Rps::Rock, Rps::Paper) | (Rps::Scissors,Rps::Scissors) => Rps::Rock,
        (Rps::Scissors, Rps::Rock) | (Rps::Paper, Rps::Paper) | (Rps::Rock, Rps::Scissors) => Rps::Paper,
        (Rps::Rock, Rps::Rock) | (Rps::Scissors , Rps::Paper) | (Rps::Paper, Rps::Scissors) => Rps::Scissors,
    };
    input.iter().map(|t| strat_to_win_score(t.1) + pick_score(case_to_pick(*t))).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = concat!(
        "A Y\n",
        "B X\n",
        "C Z\n",
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        use Rps::*;
        let parsed = crate::macros::parse_expect!(EXAMPLE, "example");
        assert_eq!(parsed, [(Rock, Paper), (Paper, Rock), (Scissors, Scissors)]);
    }

    crate::macros::make_test_for_day!(example, EXAMPLE, 15, 12);
}
