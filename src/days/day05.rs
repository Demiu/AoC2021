use std::{collections::HashSet, hash::Hash, ops::RangeInclusive};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1, sequence::separated_pair, IResult};

use crate::{parse::parse_unsigned, traits::Intersect};

type ParserOutput = Vec<Line>;
type SolverInput = [Line];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq)]
pub struct HorizontalLine {
    xs: RangeInclusive<u32>,
    y: u32,
}

#[derive(Debug, PartialEq)]
pub struct VerticalLine {
    x: u32,
    ys: RangeInclusive<u32>,
}

#[derive(Debug, PartialEq)]
pub struct DiagonalLineInc {
    start: Point,
    length: u32,
}

#[derive(Debug, PartialEq)]
pub struct DiagonalLineDec {
    start: Point,
    length: u32,
}

#[derive(Debug, PartialEq)]
pub enum Line {
    Horizontal(HorizontalLine),
    Vertical(VerticalLine),
    IncDiagonal(DiagonalLineInc),
    DecDiagonal(DiagonalLineDec),
    Generic { start: Point, end: Point },
}
use Line::{DecDiagonal, Horizontal, IncDiagonal, Vertical};

impl HorizontalLine {
    fn new(xs: RangeInclusive<u32>, y: u32) -> HorizontalLine {
        HorizontalLine { xs, y }
    }
}

impl VerticalLine {
    fn new(x: u32, ys: RangeInclusive<u32>) -> VerticalLine {
        VerticalLine { x, ys }
    }
}

impl DiagonalLineInc {
    fn new(start: Point, length: u32) -> DiagonalLineInc {
        DiagonalLineInc { start, length }
    }
}

impl DiagonalLineDec {
    fn new(start: Point, length: u32) -> DiagonalLineDec {
        DiagonalLineDec { start, length }
    }
}

impl Intersect for HorizontalLine {
    type Output = HashSet<Point>;

    fn intersect_with(&self, other: &Self) -> Option<Self::Output> {
        if self.y != other.y {
            None
        } else if let Some(xs_intersect) = self.xs.intersect_with(&other.xs) {
            let mut points = HashSet::new();
            for x in xs_intersect {
                points.insert(Point { x, y: self.y });
            }
            Some(points)
        } else {
            None
        }
    }
}

impl Intersect for VerticalLine {
    type Output = HashSet<Point>;

    fn intersect_with(&self, other: &Self) -> Option<Self::Output> {
        if self.x != other.x {
            None
        } else if let Some(ys_intersect) = self.ys.intersect_with(&other.ys) {
            let mut points = HashSet::new();
            for y in ys_intersect {
                points.insert(Point { x: self.x, y });
            }
            Some(points)
        } else {
            None
        }
    }
}

impl Intersect<HorizontalLine> for VerticalLine {
    type Output = HashSet<Point>;

    fn intersect_with(&self, other: &HorizontalLine) -> Option<Self::Output> {
        if self.ys.contains(&other.y) && other.xs.contains(&self.x) {
            Some(HashSet::from([Point {
                x: self.x,
                y: other.y,
            }]))
        } else {
            None
        }
    }
}

impl Intersect<HorizontalLine> for DiagonalLineInc {
    type Output = HashSet<Point>;

    fn intersect_with(&self, other: &HorizontalLine) -> Option<Self::Output> {
        if self.start.y >= other.y {
            let difference = self.start.y - other.y;
            let x = self.start.x + difference;
            if difference <= self.length && other.xs.contains(&x) {
                return Some(HashSet::from([Point { x, y: other.y }]));
            }
        }
        None
    }
}

impl Intersect<HorizontalLine> for DiagonalLineDec {
    type Output = HashSet<Point>;

    fn intersect_with(&self, other: &HorizontalLine) -> Option<Self::Output> {
        if self.start.y <= other.y {
            // 6 hits, if and else
            let difference = other.y - self.start.y;
            let x = self.start.x + difference;
            if difference <= self.length && other.xs.contains(&x) {
                return Some(HashSet::from([Point { x, y: other.y }]));
            }
        }
        None
    }
}

impl Intersect<VerticalLine> for DiagonalLineInc {
    type Output = HashSet<Point>;

    fn intersect_with(&self, other: &VerticalLine) -> Option<Self::Output> {
        if self.start.x > other.x {
            return None;
        } else {
            let difference = other.x - self.start.x;
            if difference <= self.length && self.start.y >= difference {
                let y = self.start.y - difference;
                if other.ys.contains(&y) {
                    return Some(HashSet::from([Point { x: other.x, y }]));
                }
            }
        }
        None
    }
}

impl Intersect<VerticalLine> for DiagonalLineDec {
    type Output = HashSet<Point>;

    fn intersect_with(&self, other: &VerticalLine) -> Option<Self::Output> {
        if self.start.x > other.x {
            return None;
        } else {
            let difference = other.x - self.start.x;
            let y = self.start.y + difference;
            if difference <= self.length && other.ys.contains(&y) {
                return Some(HashSet::from([Point { x: other.x, y }]));
            }
        }
        None
    }
}

impl Intersect for DiagonalLineInc {
    type Output = HashSet<Point>;

    fn intersect_with(&self, other: &DiagonalLineInc) -> Option<Self::Output> {
        // for increasing lines sum x+y is constant
        // if the sums aren't equal they're on separate super lines
        if self.start.x + self.start.y != other.start.x + other.start.y {
            return None; // +2
        }

        let xs1 = self.start.x..=(self.start.x + self.length);
        let xs2 = other.start.x..=(other.start.x + other.length);
        if let Some(xs) = xs1.intersect_with(&xs2) {
            let mut points = HashSet::new();
            // y can be calculated backwards from sum
            let sum = self.start.x + self.start.y;
            for x in xs {
                points.insert(Point { x, y: sum - x });
            }
            return Some(points);
        }
        None
    }
}

impl Intersect for DiagonalLineDec {
    type Output = HashSet<Point>;

    fn intersect_with(&self, other: &DiagonalLineDec) -> Option<Self::Output> {
        // for decreasing lines difference x-y is constant
        // if the differences aren't equal they're on separate super lines
        let self_sign = self.start.x >= self.start.y;
        let other_sign = other.start.x >= other.start.y;
        if self_sign != other_sign {
            return None;
        }
        if self_sign {
            if self.start.x - self.start.y != other.start.x - other.start.y {
                return None;
            }
        } else if self.start.y - self.start.x != other.start.y - other.start.x {
            return None;
        }

        let xs1 = self.start.x..=(self.start.x + self.length);
        let xs2 = other.start.x..=(other.start.x + other.length);
        if let Some(xs) = xs1.intersect_with(&xs2) {
            let mut points = HashSet::new();
            for x in xs {
                let y = if self.start.x >= self.start.y {
                    x - (self.start.x - self.start.y)
                } else {
                    x + (self.start.y - self.start.x)
                };
                points.insert(Point { x, y });
            }
            return Some(points);
        }
        None
    }
}

impl Intersect<DiagonalLineInc> for DiagonalLineDec {
    type Output = HashSet<Point>;

    fn intersect_with(&self, other: &DiagonalLineInc) -> Option<Self::Output> {
        // ly = lx + a --> a = ly - lx
        // ry = -rx + b --> b = ry + rx
        // cross point: lx = rx and ly = ry
        // x + a = -x + b --> 2x = b - a
        let b = other.start.x + other.start.y; // +1
        let x = if self.start.y >= self.start.x {
            let a = self.start.y - self.start.x;
            if a > b {
                return None; // negative x
            } else {
                let two_x = b - a;
                if two_x % 2 == 1 {
                    return None; // non-whole point
                } else {
                    two_x / 2
                }
            }
        } else {
            let minus_a = self.start.x - self.start.y;
            let two_x = b + minus_a;
            if two_x % 2 == 1 {
                return None; // non-whole point
            } else {
                two_x / 2
            }
        };
        // for decreasing lines b is constant, this allows us to deduce y
        let y = if x > b { x - b } else { b - x };

        let lxs = self.start.x..=(self.start.x + self.length);
        let lys = self.start.y..=(self.start.y + self.length);
        let rxs = other.start.x..=(other.start.x + other.length);
        let rys = (other.start.y - other.length)..=other.start.y;
        if lxs.contains(&x) && lys.contains(&y) && rxs.contains(&x) && rys.contains(&y) {
            Some(HashSet::from([Point { x, y }]))
        } else {
            None
        }
    }
}

impl Intersect for Line {
    type Output = HashSet<Point>;

    fn intersect_with(&self, other: &Line) -> Option<Self::Output> {
        match (self, other) {
            (Horizontal(h1), Horizontal(h2)) => h1.intersect_with(h2),
            (Vertical(v1), Vertical(v2)) => v1.intersect_with(v2),
            (Vertical(v), Horizontal(h)) | (Horizontal(h), Vertical(v)) => v.intersect_with(h),
            (IncDiagonal(i), Horizontal(h)) | (Horizontal(h), IncDiagonal(i)) => {
                i.intersect_with(h)
            }
            (IncDiagonal(i), Vertical(v)) | (Vertical(v), IncDiagonal(i)) => i.intersect_with(v),
            (DecDiagonal(d), Horizontal(h)) | (Horizontal(h), DecDiagonal(d)) => {
                d.intersect_with(h)
            }
            (DecDiagonal(d), Vertical(v)) | (Vertical(v), DecDiagonal(d)) => d.intersect_with(v),
            (IncDiagonal(i), DecDiagonal(d)) | (DecDiagonal(d), IncDiagonal(i)) => {
                d.intersect_with(i)
            }
            (IncDiagonal(i1), IncDiagonal(i2)) => i1.intersect_with(i2),
            (DecDiagonal(d1), DecDiagonal(d2)) => d1.intersect_with(d2),
            _ => None,
        }
    }
}

fn make_range(one: u32, two: u32) -> RangeInclusive<u32> {
    if one <= two {
        one..=two
    } else {
        two..=one
    }
}

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
    fn parse_point(input: &[u8]) -> IResult<&[u8], Point> {
        let (rest, (x, y)) = separated_pair(parse_unsigned, tag(b","), parse_unsigned)(input)?;
        Ok((rest, Point { x, y }))
    }
    fn parse_line(input: &[u8]) -> IResult<&[u8], Line> {
        let (rest, (start, end)) = separated_pair(parse_point, tag(b" -> "), parse_point)(input)?;
        Ok((rest, Line::Generic { start, end }))
    }

    let mut lines = separated_list1(tag(b"\n"), parse_line)(file)
        .map_err(|_| anyhow!("Failed parsing lines"))?
        .1;
    for line in lines.iter_mut() {
        if let Line::Generic { start, end } = line {
            match (start.x == end.x, start.y == end.y) {
                (true, true) => unreachable!("Zero length line"),
                (false, true) => {
                    *line = Horizontal(HorizontalLine::new(make_range(start.x, end.x), end.y))
                }
                (true, false) => {
                    *line = Vertical(VerticalLine::new(end.x, make_range(start.y, end.y)))
                }
                _ => {
                    *line = match (start.x < end.x, start.y < end.y) {
                        (true, true) => DecDiagonal(DiagonalLineDec::new(*start, end.x - start.x)),
                        (true, false) => IncDiagonal(DiagonalLineInc::new(*start, end.x - start.x)),
                        (false, true) => IncDiagonal(DiagonalLineInc::new(*end, start.x - end.x)),
                        (false, false) => DecDiagonal(DiagonalLineDec::new(*end, start.x - end.x)),
                    }
                }
            }
        } else {
            unreachable!()
        }
    }
    Ok(lines)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut points = HashSet::new();
    for (line1, line2) in input.iter().tuple_combinations() {
        let new_points_opt = match (line1, line2) {
            (Horizontal(_), Horizontal(_)) => line1.intersect_with(line2),
            (Vertical(_), Vertical(_)) => line1.intersect_with(line2),
            (Horizontal(_), Vertical(_)) => line1.intersect_with(line2),
            (Vertical(_), Horizontal(_)) => line1.intersect_with(line2),
            _ => None,
        };
        if let Some(new_points) = new_points_opt {
            points.extend(new_points);
        }
    }
    points.len() as u32
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let mut points = HashSet::new();
    for (line1, line2) in input.iter().tuple_combinations() {
        let new_points_opt = line1.intersect_with(line2);
        if let Some(new_points) = new_points_opt {
            points.extend(new_points);
        }
    }
    points.len() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = concat!(
        "0,9 -> 5,9\n",
        "8,0 -> 0,8\n",
        "9,4 -> 3,4\n",
        "2,2 -> 2,1\n",
        "7,0 -> 7,4\n",
        "6,4 -> 2,0\n",
        "0,9 -> 2,9\n",
        "3,4 -> 1,4\n",
        "0,0 -> 8,8\n",
        "5,5 -> 8,2\n",
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        let parsed = parse_input(EXAMPLE);
        assert!(parsed.is_ok(), "Failed parsing example input");
        let parsed = parsed.unwrap();
        assert_eq!(
            parsed[0],
            Line::Horizontal(HorizontalLine { xs: 0..=5, y: 9 })
        );
        assert_eq!(
            parsed[1],
            Line::IncDiagonal(DiagonalLineInc {
                start: Point { x: 0, y: 8 },
                length: 8
            })
        );
        assert_eq!(
            parsed[2],
            Line::Horizontal(HorizontalLine { xs: 3..=9, y: 4 })
        );
        assert_eq!(parsed[3], Line::Vertical(VerticalLine { x: 2, ys: 1..=2 }));
        assert_eq!(parsed[4], Line::Vertical(VerticalLine { x: 7, ys: 0..=4 }));
        assert_eq!(
            parsed[5],
            Line::DecDiagonal(DiagonalLineDec {
                start: Point { x: 2, y: 0 },
                length: 4
            })
        );
        assert_eq!(
            parsed[6],
            Line::Horizontal(HorizontalLine { xs: 0..=2, y: 9 })
        );
        assert_eq!(
            parsed[7],
            Line::Horizontal(HorizontalLine { xs: 1..=3, y: 4 })
        );
        assert_eq!(
            parsed[8],
            Line::DecDiagonal(DiagonalLineDec {
                start: Point { x: 0, y: 0 },
                length: 8
            })
        );
        assert_eq!(
            parsed[9],
            Line::IncDiagonal(DiagonalLineInc {
                start: Point { x: 5, y: 5 },
                length: 3
            })
        );
    }

    crate::macros::make_test_for_day!(example, EXAMPLE, 5, 12);
}
