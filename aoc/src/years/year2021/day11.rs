use std::collections::HashSet;

use anyhow::{Context, Result, anyhow};
use ndarray::Array2;
use nom::{
    bytes::complete::tag, character::complete::digit1, error::Error, multi::separated_list1,
};

type SolverInput = Array2<u8>;

const SIDE_LEN: usize = 10;

fn step(grid: &mut SolverInput) -> u32 {
    let (rows, cols) = grid.dim();
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
        let bottom_ok = y < cols - 1;
        let left_ok = x > 0;
        let right_ok = x < rows - 1;
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

    let (rows, cols) = (digits.len(), digits[0].len());

    Array2::from_shape_vec(
        (rows, cols),
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

#[cfg(test)]
mod test {
    use ndarray::{ArrayView, Axis};

    use super::*;

    const EXAMPLE_SMALL: &[u8] =
        concat!("11111\n", "19991\n", "19191\n", "19991\n", "11111\n",).as_bytes();

    const EXAMPLE_LARGE: &[u8] = concat!(
        "5483143223\n",
        "2745854711\n",
        "5264556173\n",
        "6141336146\n",
        "6357385478\n",
        "4167524645\n",
        "2176841721\n",
        "6882881134\n",
        "4846848554\n",
        "5283751526\n",
    )
    .as_bytes();

    #[test]
    fn parse_example_small() {
        let parsed = rules::parse_expect!(EXAMPLE_SMALL, "small example");
        assert_eq!(
            parsed.index_axis(Axis(0), 0),
            ArrayView::from(&[1, 1, 1, 1, 1])
        );
        assert_eq!(
            parsed.index_axis(Axis(0), 1),
            ArrayView::from(&[1, 9, 9, 9, 1])
        );
        assert_eq!(
            parsed.index_axis(Axis(0), 2),
            ArrayView::from(&[1, 9, 1, 9, 1])
        );
        assert_eq!(
            parsed.index_axis(Axis(0), 3),
            ArrayView::from(&[1, 9, 9, 9, 1])
        );
        assert_eq!(
            parsed.index_axis(Axis(0), 4),
            ArrayView::from(&[1, 1, 1, 1, 1])
        );
    }

    #[test]
    fn step_example_small() {
        let mut parsed = rules::parse_expect!(EXAMPLE_SMALL, "small example");
        // only check the first 3 lines
        step(&mut parsed);
        assert_eq!(
            parsed.index_axis(Axis(0), 0),
            ArrayView::from(&[3, 4, 5, 4, 3])
        );
        assert_eq!(
            parsed.index_axis(Axis(0), 1),
            ArrayView::from(&[4, 0, 0, 0, 4])
        );
        assert_eq!(
            parsed.index_axis(Axis(0), 2),
            ArrayView::from(&[5, 0, 0, 0, 5])
        );
        // only check the last 3 lines
        step(&mut parsed);
        assert_eq!(
            parsed.index_axis(Axis(0), 2),
            ArrayView::from(&[6, 1, 1, 1, 6])
        );
        assert_eq!(
            parsed.index_axis(Axis(0), 3),
            ArrayView::from(&[5, 1, 1, 1, 5])
        );
        assert_eq!(
            parsed.index_axis(Axis(0), 4),
            ArrayView::from(&[4, 5, 6, 5, 4])
        );
    }

    #[test]
    fn parse_example_large() {
        let parsed = rules::parse_expect!(EXAMPLE_LARGE, "large example");
        // cherrypick lines
        assert_eq!(
            parsed.index_axis(Axis(0), 0),
            ArrayView::from(&[5, 4, 8, 3, 1, 4, 3, 2, 2, 3])
        );
        assert_eq!(
            parsed.index_axis(Axis(0), 3),
            ArrayView::from(&[6, 1, 4, 1, 3, 3, 6, 1, 4, 6])
        );
        assert_eq!(
            parsed.index_axis(Axis(0), 6),
            ArrayView::from(&[2, 1, 7, 6, 8, 4, 1, 7, 2, 1])
        );
        assert_eq!(
            parsed.index_axis(Axis(0), 9),
            ArrayView::from(&[5, 2, 8, 3, 7, 5, 1, 5, 2, 6])
        );
    }

    #[test]
    fn step_example_large() {
        let mut parsed = rules::parse_expect!(EXAMPLE_LARGE, "large example");
        // cherrypick lines
        step(&mut parsed);
        assert_eq!(
            parsed.index_axis(Axis(0), 1),
            ArrayView::from(&[3, 8, 5, 6, 9, 6, 5, 8, 2, 2])
        );
        assert_eq!(
            parsed.index_axis(Axis(0), 4),
            ArrayView::from(&[7, 4, 6, 8, 4, 9, 6, 5, 8, 9])
        );
        assert_eq!(
            parsed.index_axis(Axis(0), 7),
            ArrayView::from(&[7, 9, 9, 3, 9, 9, 2, 2, 4, 5])
        );
        step(&mut parsed);
        assert_eq!(
            parsed.index_axis(Axis(0), 0),
            ArrayView::from(&[8, 8, 0, 7, 4, 7, 6, 5, 5, 5])
        );
        assert_eq!(
            parsed.index_axis(Axis(0), 4),
            ArrayView::from(&[8, 7, 0, 0, 9, 0, 8, 8, 0, 0])
        );
        assert_eq!(
            parsed.index_axis(Axis(0), 8),
            ArrayView::from(&[9, 0, 0, 0, 0, 0, 0, 8, 7, 6])
        );
    }

    rules::make_test_for_day!(example, EXAMPLE_LARGE, 1656, 195);
}
