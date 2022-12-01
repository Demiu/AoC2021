use std::{cmp::Ordering, collections::HashMap};

use anyhow::{anyhow, Result};
use ndarray::Array2;
use nom::{
    bytes::complete::tag, character::complete::digit1, error::Error, multi::separated_list1,
};
use priority_queue::PriorityQueue;

type SolverInput = Array2<u8>;

type Position = [usize; 2];

#[derive(Clone, Copy, PartialEq, Eq)]
struct PfNode {
    previous: Position,
    cost: u32,
}

impl PartialOrd for PfNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PfNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn update_queue(
    queue: &mut PriorityQueue<Position, PfNode>,
    at: Position,
    at_cost: u32,
    from: Position,
    from_cost: u32,
) {
    if let Some(current_path) = queue.get_priority(&at) {
        if at_cost + from_cost < current_path.cost {
            queue.change_priority(
                &at,
                PfNode {
                    previous: from,
                    cost: at_cost + from_cost,
                },
            );
        }
    } else {
        queue.push(
            at,
            PfNode {
                previous: from,
                cost: at_cost + from_cost,
            },
        );
    }
}

fn pathfind(grid: &SolverInput) -> u32 {
    let len_y = grid.shape()[0];
    let len_x = grid.shape()[1];

    let mut explored = HashMap::new();
    let mut queue = PriorityQueue::new();
    queue.push(
        [0, 0],
        PfNode {
            previous: [0, 0],
            cost: 0,
        },
    );

    while let Some(([y, x], path)) = queue.pop() {
        explored.insert([y, x], path.cost);

        if [y, x] == [len_y - 1, len_x - 1] {
            break; // the end
        }

        if y > 0 {
            let next_position = [y - 1, x];
            if !explored.contains_key(&next_position) {
                let next_cost = grid[next_position] as u32;
                update_queue(&mut queue, next_position, next_cost, [y, x], path.cost);
            }
        }
        if y < len_y - 1 {
            let next_position = [y + 1, x];
            if !explored.contains_key(&next_position) {
                let next_cost = grid[next_position] as u32;
                update_queue(&mut queue, next_position, next_cost, [y, x], path.cost);
            }
        }
        if x > 0 {
            let next_position = [y, x - 1];
            if !explored.contains_key(&next_position) {
                let next_cost = grid[next_position] as u32;
                update_queue(&mut queue, next_position, next_cost, [y, x], path.cost);
            }
        }
        if x < len_x - 1 {
            let next_position = [y, x + 1];
            if !explored.contains_key(&next_position) {
                let next_cost = grid[next_position] as u32;
                update_queue(&mut queue, next_position, next_cost, [y, x], path.cost);
            }
        }
    }

    explored[&[len_y - 1, len_x - 1]]
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    let lines = separated_list1::<_, _, _, Error<_>, _, _>(tag(b"\n"), digit1)(file)
        .map_err(|_| anyhow!("Failed parsing grid"))?
        .1;
    let len_y = lines.len();
    let len_x = lines[0].len();
    let mut array = Array2::default((len_y, len_x));
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            array[[y, x]] = *c - b'0';
        }
    }
    Ok(array)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let grid = input;
    pathfind(grid)
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let oglen_y = input.shape()[0];
    let oglen_x = input.shape()[1];
    let shape = (oglen_y * 5, oglen_x * 5);
    let mut grid = Array2::default(shape);
    for ((ogy, ogx), &val) in input.indexed_iter() {
        for tile_y in 0..5 {
            for tile_x in 0..5 {
                let offset_y = oglen_y * tile_y;
                let offset_x = oglen_x * tile_x;
                let new_val = (val - 1 + tile_x as u8 + tile_y as u8) % 9 + 1;
                grid[[offset_y + ogy, offset_x + ogx]] = new_val;
            }
        }
    }
    pathfind(&grid)
}
