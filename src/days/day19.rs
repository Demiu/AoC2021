use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

use anyhow::{anyhow, Context, Result};
use itertools::{iproduct, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::parse::parse_signed;

// (the meta-scanner, indicies of scanners in input to their position)
type SolverInput = (Scanner, HashMap<usize, Point>);

type Rotator = dyn Fn(i32, i32, i32) -> (i32, i32, i32);
type Metadata = (i32, i32);

const ROTATORS: [&Rotator; 24] = [
    &|x, y, z| (x, y, z),
    &|x, y, z| (x, -y, -z),
    &|x, y, z| (x, z, -y),
    &|x, y, z| (x, -z, y),
    &|x, y, z| (y, z, x),
    &|x, y, z| (y, -z, -x),
    &|x, y, z| (y, x, -z),
    &|x, y, z| (y, -x, z),
    &|x, y, z| (z, x, y),
    &|x, y, z| (z, -x, -y),
    &|x, y, z| (z, y, -x),
    &|x, y, z| (z, -y, x),
    &|x, y, z| (-x, z, y),
    &|x, y, z| (-x, -z, -y),
    &|x, y, z| (-x, y, -z),
    &|x, y, z| (-x, -y, z),
    &|x, y, z| (-y, x, z),
    &|x, y, z| (-y, -x, -z),
    &|x, y, z| (-y, z, -x),
    &|x, y, z| (-y, -z, x),
    &|x, y, z| (-z, y, x),
    &|x, y, z| (-z, -y, -x),
    &|x, y, z| (-z, x, -y),
    &|x, y, z| (-z, -x, y),
];
const REQUIRED_BEACON_MATCHES: usize = 12;
// each metadata is for a pair of beacons
// that's (12 choose 2) combinations = 66
const REQUIRED_METADATA_MATCHES: usize = 66;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone)]
pub struct Scanner {
    beacons: Vec<Point>,
    metadatas: HashMap<Metadata, Vec<(usize, usize)>>,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn rotated(self, rot: &Rotator) -> Self {
        let (x, y, z) = rot(self.x, self.y, self.z);
        Self { x, y, z }
    }

    fn manhattan_to(&self, rhs: &Point) -> i32 {
        let diff = *self - *rhs;
        diff.x.abs() + diff.y.abs() + diff.z.abs()
    }
}

impl Scanner {
    fn new(beacons: Vec<Point>) -> Scanner {
        let metadatas = generate_metadatas(&beacons);
        Scanner { beacons, metadatas }
    }

    fn add_beacon(&mut self, beacon: Point) {
        let new_beacon_i = self.beacons.len();
        for (i, b) in self.beacons.iter().enumerate() {
            let metadata = metadata(*b, beacon);
            self.metadatas
                .entry(metadata)
                .or_default()
                .push((i, new_beacon_i));
        }
        self.beacons.push(beacon);
    }

    fn merge_with(&mut self, other: &Self, offset: Point, rotation: &Rotator) {
        for b in other.beacons.iter() {
            let b = offset + b.rotated(rotation);
            if !self.beacons.contains(&b) {
                self.add_beacon(b);
            }
        }
    }
}

fn metadata(p1: Point, p2: Point) -> Metadata {
    // the metadata is norm1 distance between points (manhattan)
    // and norm2 squared (to save a sqrt, since it doesn't change anything)
    let meta1 = p1.manhattan_to(&p2);
    let diff = p1 - p2;
    let meta2 = (diff.x * diff.x) + (diff.y * diff.y) + (diff.z * diff.z);
    (meta1, meta2)
}

fn generate_metadatas(beacons: &[Point]) -> HashMap<Metadata, Vec<(usize, usize)>> {
    let mut metadatas: HashMap<_, Vec<_>> = HashMap::new();
    for ((p1i, p1), (p2i, p2)) in beacons.iter().enumerate().tuple_combinations() {
        let metadata = metadata(*p1, *p2);
        metadatas.entry(metadata).or_default().push((p1i, p2i));
    }
    metadatas
}

fn check_translation(s1: &Scanner, s2: &Scanner, offset: Point, rotation: &Rotator) -> bool {
    let mut num_matches = 0;
    for beacon in s2.beacons.iter() {
        let translated = offset + beacon.rotated(rotation);
        if s1.beacons.contains(&translated) {
            num_matches += 1;
            if num_matches == REQUIRED_BEACON_MATCHES {
                return true;
            }
        }
    }
    false
}

// returns: offset of s2 from s1 and rotation of s2 to s1's coordinate system
fn find_translation(s1: &Scanner, s2: &Scanner) -> Option<(Point, &'static Rotator)> {
    let common_metadata: Vec<_> = s1
        .metadatas
        .keys()
        .filter(|md| s2.metadatas.contains_key(md))
        .collect();
    if common_metadata.len() < REQUIRED_METADATA_MATCHES {
        return None;
    }

    for md in common_metadata {
        // unwrap because they're already known to be common
        let s1_idx_pairs = s1.metadatas.get(md).unwrap();
        let s2_idx_pairs = s2.metadatas.get(md).unwrap();
        let pairs = iproduct!(s1_idx_pairs, s2_idx_pairs);
        for (&(s1i1, s1i2), &(s2i1, s2i2)) in pairs {
            let (s1p1, s1p2) = (s1.beacons[s1i1], s1.beacons[s1i2]);
            let (s2p1, s2p2) = (s2.beacons[s2i1], s2.beacons[s2i2]);

            // case 1: assume s1p1 is s2p1, s1p2 is s2p2
            // case 2: assume s1p1 is s2p2, s1p2 is s1p1
            // there's no case 2 in input
            let matching_rots: Vec<_> = ROTATORS
                .iter()
                .filter(|f| {
                    let s2p1r = s2p1.rotated(f);
                    let s2p2r = s2p2.rotated(f);
                    (s1p1 - s1p2) == (s2p1r - s2p2r)
                })
                .collect();

            for rot in matching_rots {
                let offset = s1p1 - s2p1.rotated(rot);
                if check_translation(s1, s2, offset, rot) {
                    return Some((offset, rot));
                }
            }
        }
    }

    None
}

// Returns the combined scanner and a map of scanner indicies to their offsets
fn combine_scanners(scanners: &[Scanner]) -> Option<SolverInput> {
    let (anchor, remaining) = scanners
        .split_first()
        .expect("Solving requires at least one scanner");
    let mut anchor = anchor.clone();
    // vec of tuples with input position to scanner reference
    let mut remaining: Vec<_> = remaining
        .iter()
        .enumerate()
        .map(|(i, s)| (i + 1, s))
        .collect();

    let mut anchor_positions = HashMap::new();
    anchor_positions.insert(0, Point::new(0, 0, 0));

    while !remaining.is_empty() {
        let mut found = None;
        for (remaining_i, &(scanners_i, scanner)) in remaining.iter().enumerate() {
            if let Some((off, rot)) = find_translation(&anchor, scanner) {
                found = Some((remaining_i, scanners_i, off, rot));
                break;
            }
        }
        if let Some((remaining_idx, scanners_idx, offset, rotation)) = found {
            let other = remaining.remove(remaining_idx).1;
            anchor.merge_with(other, offset, rotation);
            anchor_positions.insert(scanners_idx, offset);
        } else {
            panic!("Failed to find match all scanners");
        }
    }

    Some((anchor, anchor_positions))
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    fn parse_beacon(input: &[u8]) -> IResult<&[u8], Point> {
        let (rest, nums) = separated_list1(tag(b","), parse_signed)(input)?;
        Ok((
            rest,
            Point {
                x: nums[0],
                y: nums[1],
                z: nums[2],
            },
        ))
    }
    fn parse_scanner(input: &[u8]) -> IResult<&[u8], Scanner> {
        let scanner_header_parser = tuple((tag(b"--- scanner "), digit1, tag(b" ---\n")));
        let beacons_parser = separated_list1(tag(b"\n"), parse_beacon);
        let (rest, beacons) = preceded(scanner_header_parser, beacons_parser)(input)?;
        Ok((rest, Scanner::new(beacons)))
    }

    let (_, scanners) = separated_list1(tag(b"\n\n"), parse_scanner)(file)
        .map_err(|_| anyhow!("Failed parsing scanners"))?;

    combine_scanners(&scanners).context(anyhow!("Failed combining scanners"))
}

pub fn solve_part1(input: &SolverInput) -> usize {
    input.0.beacons.len()
}

pub fn solve_part2(input: &SolverInput) -> i32 {
    input
        .1
        .values()
        .tuple_combinations()
        .map(|(o1, o2)| o1.manhattan_to(o2))
        .max()
        .unwrap_or(-1)
}
