use std::{cmp::Reverse, collections::BTreeSet};

use anyhow::{Result, anyhow};
use nom::{
    bytes::complete::tag,
    multi::separated_list1,
    sequence::{terminated, tuple},
};

use crate::parse::parse_unsigned;

type Coords = (u64, u64, u64);
type ParserOutput = Vec<Coords>;
type SolverInput = [Coords];

struct DistIndicies(u64, (usize, usize));

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
    separated_list1(
        tag(b"\n"),
        tuple((
            terminated(parse_unsigned, tag(b",")),
            terminated(parse_unsigned, tag(b",")),
            parse_unsigned,
        )),
    )(file)
    .map_err(|_| anyhow!("Failed parsing cells"))
    .map(|t| t.1)
}

pub fn solve_part1(input: &SolverInput) -> usize {
    let after1k = |i, _: &_| i == 1000;
    solve_gen(input, after1k).0
}

pub fn solve_part2(input: &SolverInput) -> u64 {
    let one_group = |_, groups: &[BTreeSet<_>]| groups.len() == 1 && groups[0].len() == input.len();
    let (last_a, last_b) = solve_gen(input, one_group).1;
    input[last_a].0 * input[last_b].0
}

fn solve_gen<F>(input: &SolverInput, term_cond: F) -> (usize, (usize, usize))
where
    F: Fn(usize, &[BTreeSet<usize>]) -> bool,
{
    let distance2 = |l: Coords, r: Coords| {
        l.0.abs_diff(r.0).pow(2) + l.1.abs_diff(r.1).pow(2) + l.2.abs_diff(r.2).pow(2)
    };
    let find_which_group = |groups: &[BTreeSet<_>], elem| {
        groups
            .iter()
            .enumerate()
            .find_map(|(i, grp)| grp.contains(&elem).then_some(i))
    };

    let distances = {
        let elems = input.len();
        let dist_coord_it = (0..elems).flat_map(|from| {
            ((from + 1)..elems)
                .map(move |to| DistIndicies(distance2(input[from], input[to]), (from, to)))
        });
        BTreeSet::from_iter(dist_coord_it)
    };
    let mut groups = Vec::new();
    let mut it = 0;
    let p2 = distances
        .into_iter()
        .find_map(|DistIndicies(_, (l, r))| {
            let lgp = find_which_group(&groups, l);
            let rgp = find_which_group(&groups, r);
            match (lgp, rgp) {
                (Some(gl), Some(gr)) if gl == gr => (), // Same group - NOOP
                (Some(gl), Some(gr)) => {
                    let mut l = groups.remove(gl.max(gr));
                    let mut r = groups.remove(gl.min(gr));
                    let combined = match l.len() > r.len() {
                        true => {
                            l.extend(r);
                            l
                        }
                        false => {
                            r.extend(l);
                            r
                        }
                    };
                    groups.push(combined);
                }
                (Some(g), None) => {
                    groups[g].insert(r);
                }
                (None, Some(g)) => {
                    groups[g].insert(l);
                }
                (None, None) => {
                    groups.push(BTreeSet::from([l, r]));
                }
            }
            it += 1;
            term_cond(it, &groups).then_some((l, r))
        })
        .expect("Distance iteration will terminate");
    groups.sort_by_key(|v| Reverse(v.len()));
    let p1 = groups.into_iter().map(|v| v.len()).take(3).product();
    (p1, p2)
}

impl PartialEq for DistIndicies {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for DistIndicies {}

impl PartialOrd for DistIndicies {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DistIndicies {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

#[cfg(test)]
mod test {
    use concat_with::concat_line;

    use super::{SolverInput, parse_input, solve_gen, solve_part2};

    const EXAMPLE: &[u8] = concat_line!(
        "162,817,812",
        "57,618,57",
        "906,360,560",
        "592,479,940",
        "352,342,300",
        "466,668,158",
        "542,29,236",
        "431,825,988",
        "739,650,466",
        "52,470,668",
        "216,146,977",
        "819,987,18",
        "117,168,530",
        "805,96,715",
        "346,949,466",
        "970,615,88",
        "941,993,340",
        "862,61,35",
        "984,92,344",
        "425,690,689",
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        let parsed = rules::parse_expect!(EXAMPLE, "example");
        assert_eq!(
            parsed,
            [
                (162, 817, 812),
                (57, 618, 57),
                (906, 360, 560),
                (592, 479, 940),
                (352, 342, 300),
                (466, 668, 158),
                (542, 29, 236),
                (431, 825, 988),
                (739, 650, 466),
                (52, 470, 668),
                (216, 146, 977),
                (819, 987, 18),
                (117, 168, 530),
                (805, 96, 715),
                (346, 949, 466),
                (970, 615, 88),
                (941, 993, 340),
                (862, 61, 35),
                (984, 92, 344),
                (425, 690, 689),
            ]
        );
    }

    fn solve_part1(input: &SolverInput) -> usize {
        let after10 = |i, _: &_| i == 10;
        solve_gen(input, after10).0
    }

    rules::make_test_for_day!(example, EXAMPLE, 40, 25272);
}
