use anyhow::{Context, Result, anyhow};
use ndarray::{Array2, ArrayView2, Zip, s};
use nom::{
    Err, IResult,
    bytes::complete::tag,
    error::{ErrorKind, ParseError},
    multi::{many1, separated_list1},
};

type SolverInput = Array2<bool>;

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    fn take_bool(input: &[u8]) -> IResult<&[u8], bool> {
        match input {
            [b'@', rest @ ..] => Ok((rest, true)),
            [b'.', rest @ ..] => Ok((rest, false)),
            _ => Err(Err::Error(ParseError::from_error_kind(
                input,
                ErrorKind::Fail,
            ))),
        }
    }

    let cells = separated_list1(tag(b"\n"), many1(take_bool))(file)
        .map_err(|_| anyhow!("Failed parsing cells"))?
        .1;

    // cells[0] has to exists because separated_list1 needs at least 1 line
    let (x, y) = (cells.len(), cells[0].len());

    Array2::from_shape_vec((x, y), cells.into_iter().flatten().collect())
        .context("Failed to create array of cells")
}

pub fn solve_part1(input: &SolverInput) -> usize {
    input
        .indexed_iter()
        .filter(|&(coords, &v)| can_be_removed(input.view(), coords, v))
        .count()
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let mut array = input.clone();
    let mut removed = 0;
    loop {
        let to_remove: Vec<_> = Zip::indexed(array.view())
            .par_map_collect(|coord, &v| can_be_removed(array.view(), coord, v).then_some(coord))
            .iter()
            .copied()
            .flatten()
            .collect();
        if to_remove.is_empty() {
            break removed;
        }
        for coord in to_remove {
            removed += 1;
            array[coord] = false;
        }
    }
}

fn can_be_removed(lookup: ArrayView2<bool>, (x, y): (usize, usize), val: bool) -> bool {
    let (xmax, ymax) = lookup.dim();
    val && {
        let xlo = if x == 0 { x } else { x - 1 };
        let ylo = if y == 0 { y } else { y - 1 };
        let xhi = xmax.min(x + 2);
        let yhi = ymax.min(y + 2);
        lookup
            .slice(s![xlo..xhi, ylo..yhi])
            .into_iter()
            .filter(|&&b| b)
            .count()
            .lt(&5)
    }
}

#[cfg(test)]
mod test {
    use concat_with::concat_line;

    use super::*;

    const EXAMPLE: &[u8] = concat_line!(
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    )
    .as_bytes();

    rules::make_test_for_day!(example, EXAMPLE, 13, 43);
}
