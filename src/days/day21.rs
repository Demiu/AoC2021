use anyhow::{anyhow, Result};
use nom::{sequence::{tuple, preceded}, character::complete::digit1, bytes::complete::tag, multi::separated_list1};

use crate::parse::parse_unsigned;

type SolverInput = (u32, u32);

const POS_MAX: u32 = 10;
const DICE_MAX: u32 = 100;

// Returns sum of rolls
fn roll_dice_thrice(dice: &mut u32) -> u32 {
    let roll_sum = (*dice * 3) + 6;
    let overshoots = 4 - (DICE_MAX - *dice).min(4);
    let roll_sum = roll_sum - overshoots * DICE_MAX;
    *dice = (*dice + 3) % DICE_MAX;
    roll_sum
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    let prefix_parser = tuple((tag(b"Player "), digit1, tag(b" starting position: ")));
    let line_parser = preceded(prefix_parser, parse_unsigned);

    let (_, positions) = separated_list1(tag(b"\n"), line_parser)(file)
        .map_err(|_| anyhow!("Failed parsing scanners"))?;
    if positions.len() != 2 {
        anyhow::bail!("Invalid amount of players");
    }

    Ok((positions[0], positions[1]))
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let (mut p1_pos, mut p2_pos) = (input.0 - 1, input.1 - 1);
    let (mut p1_score, mut p2_score) = (0, 0);
    let mut dice = 0;
    let mut roll_count = 0;
    let loser_score = loop {
        let rolls = roll_dice_thrice(&mut dice);
        roll_count += 3;
        p1_pos = (p1_pos + rolls) % POS_MAX;
        p1_score += p1_pos + 1;
        if p1_score >= 1000 {
            break p2_score;
        }
        let rolls = roll_dice_thrice(&mut dice);
        roll_count += 3;
        p2_pos = (p2_pos + rolls) % POS_MAX;
        p2_score += p2_pos + 1;
        if p2_score >= 1000 {
            break p1_score;
        }
    };

    roll_count * loser_score
}
