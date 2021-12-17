use std::cmp::max;

use anyhow::Result;
use nom::{
    bytes::streaming::tag,
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::parse::parse_signed;

type SolverInput = ((i32, i32), (i32, i32));

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    fn parse_range(input: &[u8]) -> IResult<&[u8], (i32, i32)> {
        separated_pair(parse_signed, tag(b".."), parse_signed)(input)
    }

    preceded(
        tag(b"target area: x="),
        separated_pair(parse_range, tag(b", y="), parse_range),
    )(file)
    .map_err(|_| anyhow::anyhow!("Failed parsing ranges"))
    .map(|t| t.1)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    // no matter the initial velocity up, we always end up at the same hight
    // with a starting velocity + 1, but in the opposite direction
    let range_y_lower = input.1 .0;
    let initial_y = -range_y_lower - 1;
    // sum of velocities from start till reaching 0 (inclusive)
    ((initial_y * (initial_y + 1)) / 2) as u32
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let ((x_min, x_max), (y_min, y_max)) = *input;
    let y_init_max = -y_min - 1;
    let mut found = 0;
    for x_init in 0..=x_max {
        for y_init in y_min..=y_init_max {
            let (mut x, mut y) = (0, 0);
            let (mut x_vel, mut y_vel) = (x_init, y_init);
            while x <= x_max && y >= y_min {
                if x >= x_min && y <= y_max {
                    found += 1;
                    break;
                }
                x += x_vel;
                y += y_vel;
                x_vel = max(x_vel - 1, 0);
                y_vel -= 1;
            }
        }
    }
    found
}
