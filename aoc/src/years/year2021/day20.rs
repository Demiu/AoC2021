use std::collections::HashSet;

use anyhow::{Result, anyhow, bail};
use nom::{
    IResult,
    bytes::complete::tag,
    multi::{many1, separated_list1},
    sequence::separated_pair,
};

type SolverInput = (Vec<bool>, Image);

const ALGORITHM_LEN: usize = 512;
const DELTAS: [(i32, i32); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub struct Image {
    non_defaults: HashSet<(i32, i32)>,
    default: bool,
}

impl Image {
    fn enhanced(&self, algo: &[bool]) -> Image {
        let new_default = self.default ^ algo[0];
        let mut considered_points = HashSet::new();
        let mut new_points = HashSet::new();

        for point in self.non_defaults.iter() {
            for d in DELTAS {
                let point = (point.0 + d.0, point.1 + d.1);

                if considered_points.contains(&point) {
                    continue;
                }
                considered_points.insert(point);

                let filter_val = compute_filter_value(point, &self.non_defaults, self.default);
                if algo[filter_val] != new_default {
                    new_points.insert(point);
                }
            }
        }
        Image {
            non_defaults: new_points,
            default: new_default,
        }
    }
}

fn compute_filter_value(
    pos: (i32, i32),
    context: &HashSet<(i32, i32)>,
    context_flipped: bool,
) -> usize {
    let mut value = 0;
    for d in DELTAS {
        let pos = (pos.0 + d.0, pos.1 + d.1);
        value <<= 1;
        if context.contains(&pos) ^ context_flipped {
            value += 1;
        }
    }
    value
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    fn take_bool(input: &[u8]) -> IResult<&[u8], bool> {
        if input.is_empty() || (input[0] != b'#' && input[0] != b'.') {
            Err(nom::Err::Error(nom::error::ParseError::from_error_kind(
                input,
                nom::error::ErrorKind::Fail,
            )))
        } else {
            let b = match input[0] {
                b'#' => true,
                b'.' => false,
                _ => unreachable!(),
            };
            Ok((&input[1..], b))
        }
    }
    fn take_bool_vec(input: &[u8]) -> IResult<&[u8], Vec<bool>> {
        many1(take_bool)(input)
    }
    let image_parser = separated_list1(tag(b"\n"), take_bool_vec);

    let (_, (algo, lines)) = separated_pair(take_bool_vec, tag(b"\n\n"), image_parser)(file)
        .map_err(|_| anyhow!("Failed parsing scanners"))?;
    if algo.len() != ALGORITHM_LEN {
        bail!("Parsed algorithm is invalid length");
    }
    if algo[0] && algo[ALGORITHM_LEN - 1] {
        bail!("Algorithm creates infinite lit points");
    }

    let mut lit_points = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, b) in line.iter().enumerate() {
            if *b {
                lit_points.insert((x as i32, y as i32));
            }
        }
    }
    let image = Image {
        non_defaults: lit_points,
        default: false,
    };

    Ok((algo, image))
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut image = input.1.enhanced(&input.0);
    for _ in 0..1 {
        image = image.enhanced(&input.0);
    }
    image.non_defaults.len() as u32
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let mut image = input.1.enhanced(&input.0);
    for _ in 0..49 {
        image = image.enhanced(&input.0);
    }
    image.non_defaults.len() as u32
}
