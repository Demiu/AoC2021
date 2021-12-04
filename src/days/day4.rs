use anyhow::{anyhow, bail, Result};
use nom::{bytes::complete::tag, multi::separated_list1};

use crate::parse::parse_unsigned;

pub struct SolverInput {
    draws: Vec<u8>,
    boards: Vec<BingoBoard>,
}

type Cell = (u8, bool);

#[derive(Clone)]
struct BingoBoard {
    grid: Vec<Cell>,
    side_len: usize,
}

impl BingoBoard {
    // Returns true if a number was marked
    fn mark_number(&mut self, number: u8) -> bool {
        for cell in self.grid.iter_mut() {
            if !cell.1 && cell.0 == number {
                cell.1 = true;
                return true;
            }
        }
        false
    }

    fn sum_unmarked(&self) -> u32 {
        self.grid
            .iter()
            .filter_map(|c| (!c.1).then(|| c.0 as u32))
            .sum()
    }

    fn is_bingo(&self) -> bool {
        self.is_bingo_horizontal() || self.is_bingo_vertical()
    }

    fn is_bingo_horizontal(&self) -> bool {
        'line: for y in 0..self.side_len {
            for x in 0..self.side_len {
                if self[[x, y]].1 == false {
                    continue 'line;
                }
            }
            return true;
        }
        false
    }

    fn is_bingo_vertical(&self) -> bool {
        'line: for x in 0..self.side_len {
            for y in 0..self.side_len {
                if self[[x, y]].1 == false {
                    continue 'line;
                }
            }
            return true;
        }
        false
    }
}

impl std::ops::Index<[usize; 2]> for BingoBoard {
    type Output = Cell;

    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        &self.grid[idx[0] + self.side_len * idx[1]]
    }
}

impl std::ops::IndexMut<[usize; 2]> for BingoBoard {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Self::Output {
        &mut self.grid[idx[0] + self.side_len * idx[1]]
    }
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    let (rest, draws) = separated_list1(tag(b","), parse_unsigned)(file)
        .map_err(move |_| anyhow!("Failed parsing the drawn numbers list"))?;

    let mut boards = vec![];
    let mut cells = vec![];
    let mut board_len = None;
    let mut current_num = None;
    let mut restidx = 0;
    while restidx < rest.len() && rest[restidx] == b'\n' {
        restidx += 1;
    }
    while restidx < rest.len() {
        match rest[restidx] {
            n @ b'0'..=b'9' => {
                let n = n - b'0';
                if current_num.is_none() {
                    current_num = Some(n);
                } else {
                    current_num = Some(current_num.unwrap() * 10 + n);
                }
            }
            b' ' => {
                if current_num.is_some() {
                    cells.push((current_num.take().unwrap(), false));
                }
            }
            b'\n' => {
                if current_num.is_none() {
                    // entire board done
                    boards.push(BingoBoard {
                        grid: cells,
                        side_len: board_len.unwrap(),
                    });
                    cells = vec![];
                } else {
                    // a line done
                    cells.push((current_num.take().unwrap(), false));
                    if board_len.is_none() {
                        // a (first) line done, set the length, it's the same for every board
                        board_len = Some(cells.len());
                    }
                }
            }
            c => bail!("Unrecognized character when parsing boards ({})", c as char),
        }
        restidx += 1;
    }

    if !cells.is_empty() {
        boards.push(BingoBoard {
            grid: cells,
            side_len: board_len.unwrap(),
        });
    }

    Ok(SolverInput { draws, boards })
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut boards = input.boards.clone();
    for number in input.draws.iter() {
        for board in boards.iter_mut() {
            if board.mark_number(*number) && board.is_bingo() {
                return board.sum_unmarked() * *number as u32;
            }
        }
    }
    0
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let mut boards = input.boards.clone();
    for number in input.draws.iter() {
        boards.iter_mut().for_each(|board| {
            board.mark_number(*number);
        });
        if boards.len() == 1 && boards[0].is_bingo() {
            return boards[0].sum_unmarked() * *number as u32;
        } else {
            boards.retain(|board| !board.is_bingo());
        }
    }
    0
}
