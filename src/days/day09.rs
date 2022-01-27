use std::collections::{HashMap, VecDeque};

use anyhow::Result;
use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::digit1, multi::separated_list1};

type ParserOutput<'a> = Vec<&'a [u8]>;
type SolverInput<'a> = [&'a [u8]];

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
    separated_list1::<_, _, _, nom::error::Error<_>, _, _>(tag(b"\n"), digit1)(file)
        .map_err(|_| anyhow::anyhow!("Failed parsing lines"))
        .map(|t| t.1)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut sum = 0;

    let leny = input.len();
    let lenx = input[0].len();
    for (y, line) in input.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            let left_le = x > 0 && input[y][x - 1] <= *value;
            let right_le = x + 1 < lenx && input[y][x + 1] <= *value;
            let up_le = y > 0 && input[y - 1][x] <= *value;
            let down_le = y + 1 < leny && input[y + 1][x] <= *value;
            if left_le || right_le || up_le || down_le {
                continue;
            } else {
                sum += (value - b'0' + 1) as u32;
            }
        }
    }
    sum
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let leny = input.len();
    let lenx = input[0].len();
    let mut position_to_basin = HashMap::new();
    let mut basin_it = 0;
    for (y, line) in input.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            if *value == b'9' {
                continue;
            }
            if !position_to_basin.contains_key(&(x, y)) {
                // flood fill from (x, y)
                let mut positions = VecDeque::new();
                positions.push_back((x, y));

                while !positions.is_empty() {
                    let (x, y) = positions.pop_front().unwrap();
                    position_to_basin.insert((x, y), basin_it);
                    if x > 0 {
                        let to_left = input[y][x - 1];
                        if to_left != b'9' && !position_to_basin.contains_key(&(x - 1, y)) {
                            positions.push_back((x - 1, y));
                        }
                    }
                    if x + 1 < lenx {
                        let to_right = input[y][x + 1];
                        if to_right != b'9' && !position_to_basin.contains_key(&(x + 1, y)) {
                            positions.push_back((x + 1, y));
                        }
                    }
                    if y > 0 {
                        let to_up = input[y - 1][x];
                        if to_up != b'9' && !position_to_basin.contains_key(&(x, y - 1)) {
                            positions.push_back((x, y - 1));
                        }
                    }
                    if y + 1 < leny {
                        let to_down = input[y + 1][x];
                        if to_down != b'9' && !position_to_basin.contains_key(&(x, y + 1)) {
                            positions.push_back((x, y + 1));
                        }
                    }
                }

                basin_it += 1;
            }
        }
    }

    let mut basin_sizes = vec![0; basin_it];
    for (_, basin) in position_to_basin {
        basin_sizes[basin] += 1;
    }

    basin_sizes
        .iter()
        .sorted()
        .rev()
        .take(3)
        .fold(1, |acc, v| acc * *v)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = concat!(
        "2199943210\n",
        "3987894921\n",
        "9856789892\n",
        "8767896789\n",
        "9899965678\n",
    ).as_bytes();

    #[test]
    fn parse_example() {
        let parsed = parse_input(EXAMPLE);
        assert!(parsed.is_ok(), "Failed parsing example input");
        assert_eq!(parsed.unwrap(), [
            b"2199943210",
            b"3987894921",
            b"9856789892",
            b"8767896789",
            b"9899965678",
        ]);
    }

    crate::macros::make_test_for_day!(example, EXAMPLE, 15, 1134);
}
