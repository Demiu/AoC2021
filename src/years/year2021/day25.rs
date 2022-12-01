use std::fmt::Display;

use anyhow::{anyhow, Context, Result};
use ndarray::{iter::AxisIterMut, parallel::prelude::*, Array2, Axis, Dim};
use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::{many1, separated_list1},
    IResult,
};

type SolverInput = Array2<Cell>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    East,
    South,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => '.',
                Cell::East => '>',
                Cell::South => 'v',
            }
        )
    }
}

fn step(state: &mut Array2<Cell>) -> bool {
    fn par_step_axis(axis: AxisIterMut<Cell, Dim<[usize; 1]>>, cell: Cell) -> bool {
        axis.into_par_iter()
            .fold(
                || false,
                |oldret, mut line| {
                    let mut ret = false;
                    let len = line.shape()[0];

                    let can_roll_around = line[0] == Cell::Empty && line[len - 1] == cell;

                    let mut it = 1;
                    while it < len {
                        if line[it] != Cell::Empty {
                            it += 1;
                            continue;
                        }
                        if line[it - 1] == cell {
                            line[it] = cell;
                            line[it - 1] = Cell::Empty;
                            ret = true;
                            it += 1;
                        }
                        it += 1;
                    }

                    if can_roll_around {
                        line[it - 1] = Cell::Empty;
                        line[0] = cell;
                    }

                    oldret || ret
                },
            )
            .reduce(|| false, |b, ob| b || ob)
    }

    par_step_axis(state.axis_iter_mut(Axis(0)), Cell::East)
        | par_step_axis(state.axis_iter_mut(Axis(1)), Cell::South)
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    fn parse_cell(input: &[u8]) -> IResult<&[u8], Cell> {
        use nom::character::complete::char;
        let (rest, sigil) = alt((char('.'), char('>'), char('v')))(input)?;
        match sigil {
            '.' => Ok((rest, Cell::Empty)),
            '>' => Ok((rest, Cell::East)),
            'v' => Ok((rest, Cell::South)),
            _ => unreachable!(),
        }
    }

    let cells = separated_list1(tag(b"\n"), many1(parse_cell))(file)
        .map_err(|_| anyhow!("Failed parsing cells"))?
        .1;

    // cells[0] has to exists because separated_list1 needs at least 1 line
    let (x, y) = (cells.len(), cells[0].len());

    Array2::from_shape_vec((x, y), cells.into_iter().flatten().collect())
        .context("Failed to create array of cells")
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut state = input.clone();
    let mut steps = 1;
    while step(&mut state) {
        steps += 1;
    }
    steps
}
