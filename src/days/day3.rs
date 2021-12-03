use anyhow::{anyhow, Result};
use nom::{bytes::complete::tag, character::complete::digit1, multi::separated_list1};

use crate::parse::parse_u64_radix;

pub struct SolverInput<'a> {
    lines: Vec<(&'a [u8], u64)>,
    line_length: usize,
}

pub fn parse_input<'a>(file: &'a [u8]) -> Result<SolverInput<'a>> {
    let line_length = file
        .iter()
        .copied()
        .take_while(move |c| *c == b'1' || *c == b'0')
        .count();
    separated_list1(tag("\n"), digit1::<_, nom::error::Error<_>>)(file)
        .map_err(move |_| anyhow!("Line parser failed"))
        .map(move |t| {
            let mut lines: Vec<(&'a [u8], u64)> =
                t.1.into_iter()
                    .map(|line| {
                        (
                            line,
                            parse_u64_radix::<'a, nom::error::Error<_>>(2)(line)
                                .unwrap()
                                .1,
                        )
                    })
                    .collect();
            lines.sort_unstable_by_key(|line_tuple| line_tuple.1);
            SolverInput { lines, line_length }
        })
}

pub fn solve_part1(input: &SolverInput) -> u64 {
    let mut counts = vec![0; input.line_length];
    for line in input.lines.iter().copied() {
        for (i, ch) in line.0.iter().enumerate() {
            match ch {
                b'1' => counts[i] += 1,
                b'0' => (),
                _ => unreachable!(),
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for count in counts {
        gamma *= 2;
        epsilon *= 2;
        if count * 2 > input.lines.len() {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    return gamma * epsilon;
}

pub fn solve_part2(input: &SolverInput) -> u64 {
    fn find_new_min_idx(s: &[(&[u8], u64)], min: u64) -> usize {
        match s.binary_search_by_key(&min, |t| t.1) {
            Ok(v) => v,
            Err(v) => v,
        }
    }
    fn find_new_max_idx(s: &[(&[u8], u64)], max: u64) -> usize {
        match s.binary_search_by_key(&max, |t| t.1) {
            Ok(v) => v,
            Err(v) => v,
        }
    }

    let mut oxyfound = None;
    let mut co2found = None;
    let (mut oxyminidx, mut oxymaxidx) = (0, input.lines.len());
    let (mut co2minidx, mut co2maxidx) = (0, input.lines.len());
    for position in 0..input.line_length {
        let left_mask = !0 << (input.line_length - position);
        let one_in_pos = 1 << (input.line_length - position - 1);

        let oxy_eligible = &input.lines[oxyminidx..oxymaxidx];
        match (oxyfound.is_some(), oxy_eligible.len()) {
            (true, _) => (),
            (false, 1) => oxyfound = Some(oxy_eligible[0].1),
            (false, 0) => panic!("Filtered all possible oxygen values"),
            (false, _) => {
                let oxy_one_cnt = oxy_eligible
                    .iter()
                    .filter(move |t| t.0[position] == b'1')
                    .count();
                if oxy_one_cnt * 2 >= oxy_eligible.len() {
                    // one most common, or equal
                    let oxymin = (oxy_eligible[0].1 & left_mask) ^ one_in_pos;
                    oxyminidx = oxyminidx + find_new_min_idx(oxy_eligible, oxymin);
                } else {
                    // zero most common
                    let oxymax = (oxy_eligible[oxy_eligible.len()-1].1 & left_mask) ^ (one_in_pos - 1);
                    oxymaxidx = oxyminidx + find_new_max_idx(oxy_eligible, oxymax);
                }
            }
        }

        let co2_eligible = &input.lines[co2minidx..co2maxidx];
        match (co2found.is_some(), co2_eligible.len()) {
            (true, _) => (),
            (false, 1) => co2found = Some(co2_eligible[0].1),
            (false, 0) => panic!("Filtered all possible co2 values"),
            (false, _) => {
                let co2_one_cnt = co2_eligible
                    .iter()
                    .filter(move |t| t.0[position] == b'1')
                    .count();
                if co2_one_cnt * 2 < co2_eligible.len() {
                    // one least common
                    let co2min = (co2_eligible[0].1 & left_mask) ^ one_in_pos;
                    co2minidx = co2minidx + find_new_min_idx(co2_eligible, co2min);
                } else {
                    // zero least common, or equal
                    let co2max = (co2_eligible[co2_eligible.len()-1].1 & left_mask) ^ (one_in_pos - 1);
                    co2maxidx = co2minidx + find_new_max_idx(co2_eligible, co2max);
                }
            }
        }

        if oxyfound.is_some() && co2found.is_some() {
            break;
        }
    }

    if oxyfound.is_none() && oxyminidx == oxymaxidx {
        oxyfound = Some(input.lines[oxyminidx].1);
    }
    if co2found.is_none() && co2minidx == co2maxidx {
        co2found = Some(input.lines[co2minidx].1);
    }

    oxyfound.expect("Oxygen value should be found")
        * co2found.expect("Oxygen value should be found")
}
