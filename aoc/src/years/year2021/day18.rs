use std::{cmp::max, fmt::Display, ops::Add};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::parse::parse_unsigned;

type ParseOutput = Vec<Element>;
type SolverInput = [Element];

const EXPLODE_LEVEL: u8 = 4;

#[derive(Clone)]
pub enum Element {
    Number(u8),
    NestedPair {
        left: Box<Element>,
        right: Box<Element>,
    },
}

impl Element {
    fn number(&self) -> Option<u8> {
        match self {
            Element::Number(n) => Some(*n),
            _ => None,
        }
    }

    fn add_leftmost(&mut self, value: u8) {
        match self {
            Element::Number(n) => *n += value,
            Element::NestedPair { left, right: _ } => left.add_leftmost(value),
        }
    }

    fn add_rightmost(&mut self, value: u8) {
        match self {
            Element::Number(n) => *n += value,
            Element::NestedPair { left: _, right } => right.add_rightmost(value),
        }
    }

    fn explode(&mut self, level: Option<u8>) -> (Option<u8>, Option<u8>, bool) {
        let level = level.unwrap_or(0);
        match self {
            Element::Number(_) => (None, None, false),
            Element::NestedPair { left, right } if level == EXPLODE_LEVEL => {
                let (left, right) = (left.number(), right.number());
                *self = Element::Number(0);
                (left, right, true)
            }
            Element::NestedPair { left, right } => {
                if let (add_left, mut add_right, true) = left.explode(Some(level + 1)) {
                    if let Some(value) = add_right.take() {
                        right.add_leftmost(value);
                    }
                    (add_left, None, true)
                } else if let (mut add_left, add_right, true) = right.explode(Some(level + 1)) {
                    if let Some(value) = add_left.take() {
                        left.add_rightmost(value);
                    }
                    (None, add_right, true)
                } else {
                    (None, None, false)
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Element::Number(n) => {
                if *n >= 10 {
                    let left = Box::new(Element::Number(*n / 2));
                    let right = Box::new(Element::Number(*n / 2 + (*n % 2)));
                    *self = Element::NestedPair { left, right };
                    true
                } else {
                    false
                }
            }
            Element::NestedPair { left, right } => left.split() || right.split(),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Element::Number(n) => *n as u32,
            Element::NestedPair { left, right } => (left.magnitude() * 3) + (right.magnitude() * 2),
        }
    }
}

impl Add for Element {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut summed = Element::NestedPair {
            left: Box::new(self),
            right: Box::new(rhs),
        };
        while summed.explode(None).2 || summed.split() {}
        summed
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Number(v) => write!(f, "{}", v),
            Element::NestedPair { left, right } => write!(f, "[{},{}]", left, right),
        }
    }
}

pub fn parse_input(file: &[u8]) -> Result<ParseOutput> {
    fn parse_element(input: &[u8]) -> IResult<&[u8], Element> {
        if let Ok((rest, value)) = parse_unsigned(input) {
            Ok((rest, Element::Number(value)))
        } else {
            let (rest, (left, right)) = delimited(
                tag(b"["),
                separated_pair(parse_element, tag(b","), parse_element),
                tag(b"]"),
            )(input)?;
            let (left, right) = (Box::new(left), Box::new(right));
            Ok((rest, Element::NestedPair { left, right }))
        }
    }

    separated_list1(tag(b"\n"), parse_element)(file)
        .map_err(|_| anyhow::anyhow!("Failed parsing pairs"))
        .map(|t| t.1)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut sum = input[0].clone();
    for rhs in input[1..].iter().cloned() {
        sum = sum + rhs;
    }
    sum.magnitude()
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let mut max_magnitude = 0;
    for vec in input.iter().permutations(2) {
        let magnitude = (vec[0].clone() + vec[1].clone()).magnitude();
        max_magnitude = max(max_magnitude, magnitude)
    }
    max_magnitude
}
