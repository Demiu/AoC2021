use std::cmp::max;

use ndarray::{s, Array2, ArrayViewMut2};
use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    error::Error,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

use crate::parse::parse_unsigned;

type SolverInput = (Array2<bool>, Vec<Fold>);

#[derive(Clone, Copy)]
pub enum Fold {
    XAxis(usize),
    YAxis(usize),
}

fn fold_array<'a>(array: &'a mut ArrayViewMut2<bool>, fold: Fold) -> ArrayViewMut2<'a, bool> {
    match fold {
        Fold::XAxis(foldx) => {
            for y in 0..array.shape()[0] {
                for x in foldx..array.shape()[1] {
                    if array[[y, x]] {
                        array[[y, foldx + foldx - x]] = true;
                    }
                }
            }
            array.slice_mut(s![.., ..foldx])
        }
        Fold::YAxis(foldy) => {
            for y in foldy..array.shape()[0] {
                for x in 0..array.shape()[1] {
                    if array[[y, x]] {
                        array[[foldy + foldy - y, x]] = true;
                    }
                }
            }
            array.slice_mut(s![..foldy, ..])
        }
    }
}

pub fn parse_input(file: &[u8]) -> anyhow::Result<SolverInput> {
    let point_parser = separated_pair::<_, usize, _, usize, Error<_>, _, _, _>(
        parse_unsigned,
        tag(b","),
        parse_unsigned,
    );
    let points_parser = separated_list1(tag(b"\n"), point_parser);
    let axis_parser = separated_pair::<_, _, _, usize, Error<_>, _, _, _>(
        one_of("xy"),
        tag(b"="),
        parse_unsigned,
    );
    let instruction_parser = preceded(tag(b"fold along "), axis_parser);
    let instructions_parser = separated_list1(tag("\n"), instruction_parser);
    let mut input_parser = separated_pair::<_, _, _, _, Error<_>, _, _, _>(
        points_parser,
        tag(b"\n\n"),
        instructions_parser,
    );

    let (points, folds) = input_parser(file)
        .map_err(|_| anyhow::anyhow!("Failed parsing input"))?
        .1;
    let size_x = max(
        points.iter().map(|t| t.0).max(),
        folds.iter().map(|t| if t.0 == 'x' { t.1 } else { 0 }).max(),
    )
    .unwrap()
        + 1;
    let size_y = max(
        points.iter().map(|t| t.1).max(),
        folds.iter().map(|t| if t.0 == 'y' { t.1 } else { 0 }).max(),
    )
    .unwrap()
        + 1;
    let mut array = Array2::default([size_y, size_x]);
    for (x, y) in points {
        array[[y, x]] = true;
    }
    let folds = folds
        .into_iter()
        .map(|t| match t.0 {
            'x' => Fold::XAxis(t.1),
            'y' => Fold::YAxis(t.1),
            _ => unreachable!(),
        })
        .collect();

    Ok((array, folds))
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut array = input.0.clone();
    if let Some(fold) = input.1.first() {
        fold_array(&mut array.view_mut(), *fold)
            .map(|b| u32::from(*b))
            .sum()
    } else {
        0
    }
}
