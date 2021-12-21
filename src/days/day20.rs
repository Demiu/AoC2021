use std::collections::HashSet;

use anyhow::{Result, anyhow};
use nom::{bytes::complete::tag, IResult, multi::{separated_list1, many1}, sequence::separated_pair};

type SolverInput = (Vec<bool>, Image);

pub struct Image {
    non_defaults: HashSet<(i32, i32)>,
    zero_default: bool,
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    fn take_bool(input: &[u8]) -> IResult<&[u8], bool> {
        if input.len() < 1 || (input[0] != b'#' && input[0] != b'.') {
            Err(nom::Err::Error(nom::error::ParseError::from_error_kind(input, nom::error::ErrorKind::Fail)))
        } else {
            let b = match input[0] {
                b'#' => true,
                b'.' => false,
                _ => unreachable!()
            };
            Ok((&input[1..], b))
        }
    }
    fn take_bool_vec(input: &[u8]) -> IResult<&[u8], Vec<bool>> {
        many1(take_bool)(input)
    }
    let image_parser = separated_list1(tag(b"\n"), take_bool_vec);

    let (_, (algo, lines)) = separated_pair(take_bool_vec, tag(b"\n\n"), image_parser)(file)
        .map_err(|_| anyhow!("Failed parsing scanners"))?;
    let mut lit_points = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, b) in line.iter().enumerate() {
            if *b {
                lit_points.insert((x as i32, y as i32));
            }
        }
    }
    let image = Image{ non_defaults: lit_points, zero_default: true };
    
    Ok((algo, image))
}
