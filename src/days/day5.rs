use std::{
    cmp::{max, min},
    collections::HashSet,
    ops::RangeInclusive,
};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1, sequence::separated_pair, IResult};

use crate::parse::parse_unsigned;

type SolverInput = Vec<Line>;

#[derive(PartialEq, Eq, Hash)]
pub struct Point {
    x: u32,
    y: u32,
}
pub enum Line {
    PointLike(Point),
    Horizontal { xs: RangeInclusive<u32>, y: u32 },
    Vertical { x: u32, ys: RangeInclusive<u32> },
    Generic { start: Point, end: Point },
}

fn parse_point(input: &[u8]) -> IResult<&[u8], Point> {
    let (rest, (x, y)) = separated_pair(parse_unsigned, tag(b","), parse_unsigned)(input)?;
    Ok((rest, Point { x, y }))
}

fn parse_line(input: &[u8]) -> IResult<&[u8], Line> {
    let (rest, (start, end)) = separated_pair(parse_point, tag(b" -> "), parse_point)(input)?;
    Ok((rest, Line::Generic { start, end }))
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    let mut lines = separated_list1(tag(b"\n"), parse_line)(file)
        .map_err(|_| anyhow!("Failed parsing lines"))
        .map(move |t| t.1)?;
    for line in lines.iter_mut() {
        if let Line::Generic { start, end } = line {
            match (start.x == end.x, start.y == end.y) {
                (true, true) => {
                    *line = Line::PointLike(Point {
                        x: start.x,
                        y: start.y,
                    })
                }
                (false, true) => {
                    *line = if start.x <= end.x {
                        Line::Horizontal {
                            xs: start.x..=end.x,
                            y: start.y,
                        }
                    } else {
                        Line::Horizontal {
                            xs: end.x..=start.x,
                            y: start.y,
                        }
                    }
                }
                (true, false) => {
                    *line = if start.y <= end.y {
                        Line::Vertical {
                            x: start.x,
                            ys: start.y..=end.y,
                        }
                    } else {
                        Line::Vertical {
                            x: start.x,
                            ys: end.y..=start.y,
                        }
                    }
                }
                _ => (),
            }
        } else {
            unreachable!()
        }
    }
    Ok(lines)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
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

    let mut positions = HashSet::new();
    for (line1, line2) in input.iter().tuple_combinations() {
        match (line1, line2) {
            (Line::Vertical { x: x1, ys: ys1 }, Line::Vertical { x: x2, ys: ys2 }) => {
                if x1 == x2 {
                    if let Some(intersect_y) = range_intersect(ys1, ys2) {
                        for y in intersect_y {
                            positions.insert(Point { x: *x1, y });
                        }
                    }
                }
            }
            (Line::Horizontal { xs: xs1, y: y1 }, Line::Horizontal { xs: xs2, y: y2 }) => {
                if y1 == y2 {
                    if let Some(intersect_x) = range_intersect(xs1, xs2) {
                        for x in intersect_x {
                            positions.insert(Point { x, y: *y1 });
                        }
                    }
                }
            }
            (Line::Horizontal { xs, y }, Line::Vertical { x, ys }) => {
                if xs.contains(x) && ys.contains(y) {
                    positions.insert(Point { x: *x, y: *y });
                }
            }
            (Line::Vertical { x, ys }, Line::Horizontal { xs, y }) => {
                if xs.contains(x) && ys.contains(y) {
                    positions.insert(Point { x: *x, y: *y });
                }
            }
            _ => (),
        }
    }
    positions.len() as u32
}
