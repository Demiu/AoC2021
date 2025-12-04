use std::ops::RangeInclusive;

use anyhow::{anyhow, Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

use crate::{parse::parse_range_signed, traits::Intersect};

type ParserOutput = Vec<RebootStep>;
type SolverInput = [RebootStep];
type CoordInt = i64;

const SMALL_LIMIT: CoordInt = 100;

#[derive(Clone)]
struct Cuboid {
    range_x: RangeInclusive<CoordInt>,
    range_y: RangeInclusive<CoordInt>,
    range_z: RangeInclusive<CoordInt>,
}

pub struct RebootStep {
    is_on: bool,
    volume: Cuboid,
}

impl Cuboid {
    fn is_small(&self) -> bool {
        self.range_x.start().abs() < SMALL_LIMIT
            && self.range_x.end().abs() < SMALL_LIMIT
            && self.range_y.start().abs() < SMALL_LIMIT
            && self.range_y.end().abs() < SMALL_LIMIT
            && self.range_z.start().abs() < SMALL_LIMIT
            && self.range_z.end().abs() < SMALL_LIMIT
    }

    fn volume(&self) -> CoordInt {
        (self.range_x.end() - self.range_x.start() + 1)
            * (self.range_y.end() - self.range_y.start() + 1)
            * (self.range_z.end() - self.range_z.start() + 1)
    }
}

impl Intersect for Cuboid {
    type Output = Cuboid;

    fn intersect_with(&self, other: &Self) -> Option<Self::Output> {
        let overlap_x = self.range_x.intersect_with(&other.range_x);
        let overlap_y = self.range_y.intersect_with(&other.range_y);
        let overlap_z = self.range_z.intersect_with(&other.range_z);
        match (overlap_x, overlap_y, overlap_z) {
            (Some(rx), Some(ry), Some(rz)) => Some(Cuboid {
                range_x: rx,
                range_y: ry,
                range_z: rz,
            }),
            _ => None,
        }
    }
}

fn steps_volume(steps: &[RebootStep]) -> u64 {
    let mut positive_cuboids = vec![];
    let mut negative_cuboids = vec![];

    for step in steps {
        let new_negative = step.volume.intersect_with(&positive_cuboids[..]);
        let new_positive = step.volume.intersect_with(&negative_cuboids[..]);
        if let Some(to_ext) = new_negative {
            negative_cuboids.extend(to_ext.into_iter().flatten());
        }
        if let Some(to_ext) = new_positive {
            positive_cuboids.extend(to_ext.into_iter().flatten());
        }

        if step.is_on {
            positive_cuboids.push(step.volume.clone());
        }
    }

    let pos_volume: u64 = positive_cuboids.iter().map(|c| c.volume() as u64).sum();
    let neg_volume: u64 = negative_cuboids.iter().map(|c| c.volume() as u64).sum();
    pos_volume - neg_volume
}

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
    {
        let xyz_alt = alt((tag(b"x="), tag(b"y="), tag(b"z=")));
        let range_eq = preceded(xyz_alt, parse_range_signed);
        let ranges = separated_list1(tag(b","), range_eq);
        let on_off_alt = alt((tag(b"on"), tag(b"off")));
        let line = separated_pair(on_off_alt, tag(b" "), ranges);
        separated_list1(tag(b"\n"), line)(file)
    }
    .map_err(|_| anyhow!("Failed parsing ranges"))?
    .1
    .into_iter()
    .map(|(onoff, mut ranges)| {
        if ranges.len() != 3 {
            Err(anyhow!("Invalid number of ranges on line"))
        } else {
            let is_on = match onoff {
                b"on" => true,
                b"off" => false,
                _ => unreachable!(), // already filtered out by the parse
            };
            Ok(RebootStep {
                is_on,
                volume: Cuboid {
                    range_z: ranges.pop().unwrap(),
                    range_y: ranges.pop().unwrap(),
                    range_x: ranges.pop().unwrap(),
                },
            })
        }
    })
    .collect::<Result<Vec<_>>>()
    .context("Failed mapping lines into RebootStep's")
}

pub fn solve_part1(input: &SolverInput) -> u64 {
    let small_end = input.partition_point(|step| step.volume.is_small());
    steps_volume(&input[..small_end])
}

pub fn solve_part2(input: &SolverInput) -> u64 {
    steps_volume(input)
}
