use std::collections::HashSet;

use anyhow::{anyhow, Context, Result};
use ndarray::Array2;
use nom::{
    bytes::complete::tag, character::complete::digit1, error::Error, multi::separated_list1,
};

type SolverInput = Array2<u8>;

const SIDE_LEN: usize = 10;

fn step(grid: &mut SolverInput) -> u32 {
    let mut flashes = 0;
    let mut to_flash = HashSet::new();
    grid.indexed_iter_mut().for_each(|((y, x), v)| {
        *v += 1;
        if *v > 9 {
            to_flash.insert([y, x]);
        }
    });
    let propagate_flash = |g: &mut SolverInput, hs: &mut HashSet<_>, y, x| {
        // 0 flashed already and they keep their value
        let cell = &mut g[[y, x]];
        if *cell != 0 {
            *cell += 1;
            if *cell == 10 {
                hs.insert([y, x]);
            }
        }
    };
    while let Some(&coords) = to_flash.iter().next() {
        to_flash.remove(&coords);
        let [y, x] = coords;
        flashes += 1;
        grid[[y, x]] = 0;
        let top_ok = y > 0;
        let bottom_ok = y < SIDE_LEN - 1;
        let left_ok = x > 0;
        let right_ok = x < SIDE_LEN - 1;
        if left_ok {
            propagate_flash(grid, &mut to_flash, y, x - 1);
            if top_ok {
                propagate_flash(grid, &mut to_flash, y - 1, x - 1);
            }
            if bottom_ok {
                propagate_flash(grid, &mut to_flash, y + 1, x - 1);
            }
        }
        if right_ok {
            propagate_flash(grid, &mut to_flash, y, x + 1);
            if top_ok {
                propagate_flash(grid, &mut to_flash, y - 1, x + 1);
            }
            if bottom_ok {
                propagate_flash(grid, &mut to_flash, y + 1, x + 1);
            }
        }
        if top_ok {
            propagate_flash(grid, &mut to_flash, y - 1, x);
        }
        if bottom_ok {
            propagate_flash(grid, &mut to_flash, y + 1, x);
        }
    }
    flashes
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    let digits = separated_list1::<_, _, _, Error<_>, _, _>(tag(b"\n"), digit1)(file)
        .map_err(|_| anyhow!("Failed parsing digits"))?
        .1;
    Array2::from_shape_vec(
        (SIDE_LEN, SIDE_LEN),
        digits.into_iter().flatten().map(|c| *c - b'0').collect(),
    )
    .context("Failed creating the octopus array")
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut grid = input.clone();
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut grid);
    }
    flashes
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let mut grid = input.clone();
    let mut steps = 0;
    while step(&mut grid) != (SIDE_LEN * SIDE_LEN) as u32 {
        steps += 1;
    }
    steps + 1
}
