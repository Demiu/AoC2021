use anyhow::Result;
use nom::{
    bytes::complete::tag, character::complete::alpha1, multi::separated_list1,
    sequence::separated_pair, IResult,
};

type EntryVec<'a> = Vec<&'a [u8]>;
type LineTuple<'a> = (EntryVec<'a>, EntryVec<'a>);
type ParserOutput<'a> = Vec<LineTuple<'a>>;
type SolverInput<'a> = [LineTuple<'a>];

mod segments {
    pub const TOP: u8 = 1 << 0;
    pub const LEFT_TOP: u8 = 1 << 1;
    pub const RIGHT_TOP: u8 = 1 << 2;
    pub const MID: u8 = 1 << 3;
    pub const LEFT_BOT: u8 = 1 << 4;
    pub const RIGHT_BOT: u8 = 1 << 5;
    pub const BOT: u8 = 1 << 6;

    pub const COUNT: usize = 7;

    pub const ANY: u8 = !0;
    pub const ZERO: u8 = TOP | LEFT_TOP | RIGHT_TOP | LEFT_BOT | RIGHT_BOT | BOT;
    pub const ONE: u8 = RIGHT_TOP | RIGHT_BOT;
    pub const TWO: u8 = TOP | RIGHT_TOP | MID | LEFT_BOT | BOT;
    pub const THREE: u8 = TOP | RIGHT_TOP | MID | RIGHT_BOT | BOT;
    pub const FOUR: u8 = LEFT_TOP | RIGHT_TOP | MID | RIGHT_BOT;
    pub const FIVE: u8 = TOP | LEFT_TOP | MID | RIGHT_BOT | BOT;
    pub const SIX: u8 = TOP | LEFT_TOP | MID | LEFT_BOT | RIGHT_BOT | BOT;
    pub const SEVEN: u8 = TOP | RIGHT_TOP | RIGHT_BOT;
    pub const EIGHT: u8 = TOP | LEFT_TOP | RIGHT_TOP | MID | LEFT_BOT | RIGHT_BOT | BOT;
    pub const NINE: u8 = TOP | LEFT_TOP | RIGHT_TOP | MID | RIGHT_BOT | BOT;
}
type NamedSegments = [u8; segments::COUNT];

// segments with the name in pattern - restrict their mask to mask
// segments with the name not in pattern - restrict their mask to NOT mask
fn mask_on_segments_for_pattern(named_segments: &mut NamedSegments, mask: u8, pattern: &[u8]) {
    for (i, ns) in named_segments.iter_mut().enumerate() {
        if pattern.contains(&(i as u8 + b'a')) {
            *ns &= mask;
        } else {
            *ns &= !mask;
        }
    }
}

fn resolve_conflict(
    named_segments: &mut NamedSegments,
    present_mask: u8,
    not_present_mask: u8,
    patterns: &[&[u8]],
    pattern_len: usize,
) {
    let both_mask = present_mask | not_present_mask;
    let mut both_masks: Vec<_> = named_segments
        .iter_mut()
        .enumerate()
        .filter(|t| (*t.1 & both_mask) == both_mask)
        .map(|t| (t.0 as u8 + b'a', t.1))
        .collect();
    let (chunk0, chunk1) = both_masks.split_at_mut(1);
    // tuples of (name, mask)
    let named_seg0 = &mut chunk0[0];
    let named_seg1 = &mut chunk1[0];

    for pattern in patterns {
        if pattern.len() == pattern_len {
            match (
                pattern.contains(&named_seg0.0),
                pattern.contains(&named_seg1.0),
            ) {
                (true, false) => {
                    *named_seg0.1 &= present_mask;
                    *named_seg1.1 &= not_present_mask;
                }
                (false, true) => {
                    *named_seg0.1 &= not_present_mask;
                    *named_seg1.1 &= present_mask;
                }
                _ => (),
            }
        }
    }
}

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
    fn parse_character_sequences(input: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
        separated_list1(tag(b" "), alpha1)(input)
    }
    fn parse_entry_line(input: &[u8]) -> IResult<&[u8], LineTuple> {
        separated_pair(
            parse_character_sequences,
            tag(b" | "),
            parse_character_sequences,
        )(input)
    }

    let entries = separated_list1(tag(b"\n"), parse_entry_line)(file)
        .map_err(|_| anyhow::anyhow!("Failed parsing combinations and output pairs"))?
        .1;
    Ok(entries)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut found = 0;
    for line in input {
        for output in line.1.iter() {
            match output.len() {
                2 | 3 | 4 | 7 => found += 1,
                _ => (),
            }
        }
    }
    found
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let mut sum = 0;
    for entry in input {
        let mut possible_values = [segments::ANY; segments::COUNT];
        // do the guarantees
        for pattern in entry.0.iter() {
            match pattern.len() {
                2 => mask_on_segments_for_pattern(&mut possible_values, segments::ONE, pattern),
                3 => mask_on_segments_for_pattern(&mut possible_values, segments::SEVEN, pattern),
                4 => mask_on_segments_for_pattern(&mut possible_values, segments::FOUR, pattern),
                _ => (),
            }
        }
        // after guarantees we're a left with 1 certain named segment - TOP
        // and three sets of either/ors:
        // LEFT_TOP or MID, LEFT_BOT or BOT, RIGHT_TOP or RIGHT_BOT
        // they all can be deduced with the 6-length patterns of 0, 6 and 9

        // LEFT_TOP or MID - can be deduced with the 0
        // MID will be the missing segment
        resolve_conflict(
            &mut possible_values,
            segments::LEFT_TOP,
            segments::MID,
            &entry.0,
            6,
        );

        // LEFT_BOT or BOT - can be deduced with the 9
        // LEFT_BOT wil be the missing segment
        resolve_conflict(
            &mut possible_values,
            segments::BOT,
            segments::LEFT_BOT,
            &entry.0,
            6,
        );

        // RIGHT_TOP or RIGHT_BOT - now it can be deduced with the 6
        // RIGHT_TOP will be the missing segment
        resolve_conflict(
            &mut possible_values,
            segments::RIGHT_BOT,
            segments::RIGHT_TOP,
            &entry.0,
            6,
        );

        let mut value = 0;
        for output in entry.1.iter() {
            value *= 10;

            let mut value_mask = 0;
            for name in output.iter() {
                value_mask |= possible_values[(name - b'a') as usize];
            }
            value += match value_mask {
                segments::ZERO => 0,
                segments::ONE => 1,
                segments::TWO => 2,
                segments::THREE => 3,
                segments::FOUR => 4,
                segments::FIVE => 5,
                segments::SIX => 6,
                segments::SEVEN => 7,
                segments::EIGHT => 8,
                segments::NINE => 9,
                _ => unreachable!(),
            };
        }

        sum += value;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = concat!(
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | ",
        "fdgacbe cefdb cefbgd gcbe\n",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | ",
        "fcgedb cgb dgebacf gc\n",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | ",
        "cg cg fdcagb cbg\n",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | ",
        "efabcd cedba gadfec cb\n",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | ",
        "gecf egdcabf bgf bfgea\n",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | ",
        "gebdcfa ecba ca fadegcb\n",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | ",
        "cefg dcbef fcge gbcadfe\n",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ",
        "ed bcgafe cdgba cbgef\n",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | ",
        "gbdfcae bgc cg cgb\n",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | ",
        "fgae cfgab fg bagce\n",
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        let parsed = crate::macros::parse_expect!(EXAMPLE, "example");
        // Only check 0th, 4th and last line
        {
            let left = [
                "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd",
                "edb",
            ]
            .map(|s| s.as_bytes());
            let right = ["fdgacbe", "cefdb", "cefbgd", "gcbe"].map(|s| s.as_bytes());
            assert_eq!(parsed[0].0, left);
            assert_eq!(parsed[0].1, right);
        }
        {
            let left = [
                "aecbfdg", "fbg", "gf", "bafeg", "dbefa", "fcge", "gcbea", "fcaegb", "dgceab",
                "fcbdga",
            ]
            .map(|s| s.as_bytes());
            let right = ["gecf", "egdcabf", "bgf", "bfgea"].map(|s| s.as_bytes());
            assert_eq!(parsed[4].0, left);
            assert_eq!(parsed[4].1, right);
        }
        {
            let left = [
                "gcafb", "gcf", "dcaebfg", "ecagb", "gf", "abcdeg", "gaef", "cafbge", "fdbac",
                "fegbdc",
            ]
            .map(|s| s.as_bytes());
            let right = ["fgae", "cfgab", "fg", "bagce"].map(|s| s.as_bytes());
            assert_eq!(parsed[9].0, left);
            assert_eq!(parsed[9].1, right);
        }
    }

    crate::macros::make_test_for_day!(example, EXAMPLE, 26, 61229);
}
