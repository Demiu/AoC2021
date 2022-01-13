use std::ops::RangeInclusive;

use anyhow::{anyhow, Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

use crate::{parse::parse_range_signed, traits::Intersect};

type SolverInput = Vec<RebootStep>;
type CoordInt = i32;

const SMALL_LIMIT: i32 = 100;

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

    fn volume(&self) -> i32 {
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

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
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

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut positive_cuboids = vec![];
    let mut negative_cuboids = vec![];

    let small_end = input.partition_point(|step| step.volume.is_small());
    for step in &input[..small_end] {
        let mut derived_negative = vec![];
        for cuboid in &positive_cuboids {
            if let Some(overlap) = step.volume.intersect_with(cuboid) {
                derived_negative.push(overlap);
            }
        }

        let mut derived_positive = vec![];
        for cuboid in &negative_cuboids {
            if let Some(overlap) = step.volume.intersect_with(cuboid) {
                derived_positive.push(overlap);
            }
        }

        negative_cuboids.extend(derived_negative);
        positive_cuboids.extend(derived_positive);

        if step.is_on {
            positive_cuboids.push(step.volume.clone());
        }
    }

    let pos_volume: i32 = positive_cuboids.iter().map(Cuboid::volume).sum();
    let neg_volume: i32 = negative_cuboids.iter().map(Cuboid::volume).sum();
    (pos_volume - neg_volume) as u32
}
