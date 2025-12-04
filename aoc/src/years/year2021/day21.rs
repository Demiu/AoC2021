use std::collections::HashMap;

use anyhow::{anyhow, Result};
use itertools::iproduct;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    multi::separated_list1,
    sequence::{preceded, tuple},
};

use crate::parse::parse_unsigned;

type SolverInput = (u32, u32);
// Game status (pos1, score1, pos2, score2) to (wins1, wins2)
type MemoizedTurns = HashMap<GameState, (u64, u64)>;

const POS_MAX: u32 = 10;
const DICE_MAX: u32 = 100;
const THRESHOLD_PART1: u32 = 1000;
const THRESHOLD_PART2: u32 = 21;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct PlayerState {
    position: u32,
    score: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GameState {
    player1: PlayerState,
    player2: PlayerState,
}

impl GameState {
    fn flipped(&self) -> Self {
        GameState {
            player1: self.player2,
            player2: self.player1,
        }
    }
}

// Returns (sum of rolls, new dice)
fn roll_dice_thrice(dice: u32) -> (u32, u32) {
    let roll_sum = (dice * 3) + 6;
    let overshoots = 4 - (DICE_MAX - dice).min(4);
    let roll_sum = roll_sum - overshoots * DICE_MAX;
    let new_dice = (dice + 3) % DICE_MAX;
    (roll_sum, new_dice)
}

fn player_turn(state: &mut PlayerState, rolls_sum: u32) {
    state.position = (state.position + rolls_sum) % POS_MAX;
    state.score += state.position + 1;
}

fn player_turn_part1(state: &mut PlayerState, dice: &mut u32, roll_count: &mut u32) {
    let (roll_sum, new_dice) = roll_dice_thrice(*dice);
    player_turn(state, roll_sum);
    *dice = new_dice;
    *roll_count += 3;
}

fn player_superturn(state: GameState, memoized_turns: &mut MemoizedTurns) -> (u64, u64) {
    let (mut p1wins, mut p2wins) = (0, 0);
    for (r1, r2, r3) in iproduct!(0..3, 0..3, 0..3) {
        let mut case_state = state;
        let rolls_sum = r1 + r2 + r3 + 3;
        player_turn(&mut case_state.player1, rolls_sum);
        if case_state.player1.score >= THRESHOLD_PART2 {
            p1wins += 1;
        } else {
            let new_state = case_state.flipped();
            let p2p1wins = match memoized_turns.get(&new_state) {
                Some(&v) => v,
                None => player_superturn(new_state, memoized_turns),
            };
            p1wins += p2p1wins.1;
            p2wins += p2p1wins.0;
        }
    }
    memoized_turns.insert(state, (p1wins, p2wins));
    (p1wins, p2wins)
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
    let mut state = GameState {
        player1: PlayerState {
            position: input.0 - 1,
            score: 0,
        },
        player2: PlayerState {
            position: input.1 - 1,
            score: 0,
        },
    };
    let mut dice = 0;
    let mut roll_count = 0;
    let loser_score = loop {
        player_turn_part1(&mut state.player1, &mut dice, &mut roll_count);
        if state.player1.score >= THRESHOLD_PART1 {
            break state.player1.score;
        }
        player_turn_part1(&mut state.player2, &mut dice, &mut roll_count);
        if state.player2.score >= THRESHOLD_PART1 {
            break state.player2.score;
        }
    };

    roll_count * loser_score
}

pub fn solve_part2(input: &SolverInput) -> u64 {
    let state = GameState {
        player1: PlayerState {
            position: input.0 - 1,
            score: 0,
        },
        player2: PlayerState {
            position: input.1 - 1,
            score: 0,
        },
    };

    let mut turn_memory = HashMap::new();
    let (p1_wins, p2_wins) = player_superturn(state, &mut turn_memory);
    u64::max(p1_wins, p2_wins)
}
