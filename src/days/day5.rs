use std::{
    cmp::{max, min},
    collections::HashSet,
    hash::Hash,
    ops::RangeInclusive,
};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1, sequence::separated_pair, IResult};

use crate::parse::parse_unsigned;

type SolverInput = Vec<Line>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: u32,
    y: u32,
}

trait Intersect<T> {
    fn intersect_with(&self, other: &T) -> Option<HashSet<Point>>;
}

pub struct HorizontalLine {
    xs: RangeInclusive<u32>,
    y: u32,
}

pub struct VerticalLine {
    x: u32,
    ys: RangeInclusive<u32>,
}

pub struct DiagonalLineInc {
    start: Point,
    length: u32,
}

pub struct DiagonalLineDec {
    start: Point,
    length: u32,
}

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

impl Intersect<HorizontalLine> for HorizontalLine {
    fn intersect_with(&self, other: &HorizontalLine) -> Option<HashSet<Point>> {
        if self.y != other.y {
            None
        } else if let Some(xs_intersect) = range_intersect(&self.xs, &other.xs) {
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

impl Intersect<VerticalLine> for VerticalLine {
    fn intersect_with(&self, other: &VerticalLine) -> Option<HashSet<Point>> {
        if self.x != other.x {
            None
        } else if let Some(ys_intersect) = range_intersect(&self.ys, &other.ys) {
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
    fn intersect_with(&self, other: &HorizontalLine) -> Option<HashSet<Point>> {
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
    fn intersect_with(&self, other: &HorizontalLine) -> Option<HashSet<Point>> {
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
    fn intersect_with(&self, other: &HorizontalLine) -> Option<HashSet<Point>> {
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
    fn intersect_with(&self, other: &VerticalLine) -> Option<HashSet<Point>> {
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
    fn intersect_with(&self, other: &VerticalLine) -> Option<HashSet<Point>> {
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

impl Intersect<DiagonalLineInc> for DiagonalLineInc {
    fn intersect_with(&self, other: &DiagonalLineInc) -> Option<HashSet<Point>> {
        // for increasing lines sum x+y is constant
        // if the sums aren't equal they're on separate super lines
        if self.start.x + self.start.y != other.start.x + other.start.y {
            return None; // +2
        }

        let xs1 = self.start.x..=(self.start.x + self.length);
        let xs2 = other.start.x..=(other.start.x + other.length);
        if let Some(xs) = range_intersect(&xs1, &xs2) {
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

impl Intersect<DiagonalLineDec> for DiagonalLineDec {
    fn intersect_with(&self, other: &DiagonalLineDec) -> Option<HashSet<Point>> {
        // for decreasing lines difference x-y is constant
        // if the differences aren't equal they're on separate super lines
        if self.start.x >= self.start.y {
            if other.start.x < other.start.y {
                return None;
            } else if self.start.x - self.start.y != other.start.x - other.start.y {
                return None;
            }
        } else {
            if other.start.x >= other.start.y {
                return None;
            } else if self.start.y - self.start.x != other.start.y - other.start.x {
                return None;
            }
        }

        let xs1 = self.start.x..=(self.start.x + self.length);
        let xs2 = other.start.x..=(other.start.x + other.length);
        if let Some(xs) = range_intersect(&xs1, &xs2) {
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
    fn intersect_with(&self, other: &DiagonalLineInc) -> Option<HashSet<Point>> {
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
            return Some(HashSet::from([Point { x, y }]));
        } else {
            None
        }
    }
}

/*
// reverse implement intersect
impl<X, Y> Intersect<X> for Y
where X: Intersect<Y> {
    fn intersect_with(&self, other: &X) -> Option<HashSet<Point>> {
        other.intersect_with(self)
    }
}*/

impl Intersect<Line> for Line {
    fn intersect_with(&self, other: &Line) -> Option<HashSet<Point>> {
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

fn range_intersect<Idx: Ord + Copy>(
    l: &RangeInclusive<Idx>,
    r: &RangeInclusive<Idx>,
) -> Option<RangeInclusive<Idx>> {
    if l.start() > r.end() || r.start() > l.end() {
        None
    } else {
        Some(max(*l.start(), *r.start())..=min(*l.end(), *r.end()))
    }
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
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
