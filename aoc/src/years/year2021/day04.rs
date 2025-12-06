use anyhow::{Result, anyhow, bail};
use nom::{bytes::complete::tag, multi::separated_list1};

use crate::parse::parse_unsigned;

pub struct SolverInput {
    draws: Vec<u8>,
    boards: Vec<BingoBoard>,
}

type Cell = (u8, bool);

#[derive(Clone, Debug)]
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
            .filter_map(|c| (!c.1).then_some(c.0 as u32))
            .sum()
    }

    fn is_bingo(&self) -> bool {
        self.is_bingo_horizontal() || self.is_bingo_vertical()
    }

    fn is_bingo_horizontal(&self) -> bool {
        'line: for y in 0..self.side_len {
            for x in 0..self.side_len {
                if !self[[x, y]].1 {
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
                if !self[[x, y]].1 {
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
        .map_err(|_| anyhow!("Failed parsing the drawn numbers list"))?;

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

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = concat!(
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n",
        "\n",
        "22 13 17 11  0\n",
        " 8  2 23  4 24\n",
        "21  9 14 16  7\n",
        " 6 10  3 18  5\n",
        " 1 12 20 15 19\n",
        "\n",
        " 3 15  0  2 22\n",
        " 9 18 13 17  5\n",
        "19  8  7 25 23\n",
        "20 11 10 24  4\n",
        "14 21 16 12  6\n",
        "\n",
        "14 21 17 24  4\n",
        "10 16 15  9 19\n",
        "18  8 23 26 20\n",
        "22 11 13  6  5\n",
        " 2  0 12  3  7\n",
    )
    .as_bytes();

    #[test]
    fn parse_example_draws() {
        let parsed = rules::parse_expect!(EXAMPLE, "example");
        let desired_draws = [
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        assert_eq!(parsed.draws, desired_draws);
    }

    #[test]
    fn parse_example_boards() {
        let parsed = rules::parse_expect!(EXAMPLE, "example");

        let desired_boards = [
            [
                22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20,
                15, 19,
            ],
            [
                3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16,
                12, 6,
            ],
            [
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ],
        ]
        .map(|board| board.map(|num| (num, false)));
        assert_eq!(parsed.boards.len(), desired_boards.len());
        for i in 0..parsed.boards.len() {
            assert_eq!(
                parsed.boards[i].grid, desired_boards[i],
                "Board #{} parsed improperly",
                i
            );
            assert_eq!(parsed.boards[i].side_len, 5);
        }
    }

    rules::make_test_for_day!(example, EXAMPLE, 4512, 1924);
}
