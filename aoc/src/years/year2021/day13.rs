use std::collections::HashSet;

use anyhow::{Result, anyhow};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

use crate::parse::parse_unsigned;

type SolverInput = (HashSet<(u32, u32)>, Vec<Fold>);

#[derive(Clone, Copy)]
pub enum Fold {
    XAxis(u32),
    YAxis(u32),
}

fn fold_paper(paper: &mut HashSet<(u32, u32)>, fold: Fold) {
    let mut to_remove = vec![];
    let mut modified_points = vec![];
    match fold {
        Fold::XAxis(foldx) => {
            for (x, y) in paper.iter() {
                if *x > foldx {
                    to_remove.push((*x, *y));
                    modified_points.push((foldx + foldx - x, *y));
                }
            }
        }
        Fold::YAxis(foldy) => {
            for (x, y) in paper.iter() {
                if *y > foldy {
                    to_remove.push((*x, *y));
                    modified_points.push((*x, foldy + foldy - y));
                }
            }
        }
    }
    for p in to_remove {
        paper.remove(&p);
    }
    for p in modified_points {
        paper.insert(p);
    }
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    let point_parser = separated_pair(parse_unsigned, tag(b","), parse_unsigned);
    let points_parser = separated_list1(tag(b"\n"), point_parser);
    let axis_parser = separated_pair(one_of("xy"), tag(b"="), parse_unsigned);
    let instruction_parser = preceded(tag(b"fold along "), axis_parser);
    let instructions_parser = separated_list1(tag("\n"), instruction_parser);
    let mut input_parser = separated_pair(points_parser, tag(b"\n\n"), instructions_parser);

    let (points, folds) = input_parser(file)
        .map_err(|_| anyhow!("Failed parsing input"))?
        .1;
    let mut point_set = HashSet::new();
    for p in points {
        point_set.insert(p);
    }
    let folds = folds
        .into_iter()
        .map(|t| match t.0 {
            'x' => Fold::XAxis(t.1),
            'y' => Fold::YAxis(t.1),
            _ => unreachable!(),
        })
        .collect();

    Ok((point_set, folds))
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut paper = input.0.clone();
    if let Some(fold) = input.1.first() {
        fold_paper(&mut paper, *fold);
        paper.len() as u32
    } else {
        0
    }
}

pub fn solve_part2(input: &SolverInput) -> String {
    let mut paper = input.0.clone();
    for fold in input.1.iter() {
        fold_paper(&mut paper, *fold);
    }

    let xmax = paper.iter().map(|t| t.0).max().unwrap_or(0) as usize + 1;
    let ymax = paper.iter().map(|t| t.1).max().unwrap_or(0) as usize + 1;
    let mut array = vec![vec![false; xmax]; ymax];
    for (x, y) in paper {
        array[y as usize][x as usize] = true;
    }

    array
        .into_iter()
        .map(|line| line.into_iter().map(|c| if c { '#' } else { ' ' }).join(""))
        .join("\n")
}
