use nom::{bytes::complete::tag, error::{self, make_error}, IResult, branch::alt, sequence::{separated_pair, terminated}, character::complete::{digit1, char}, multi::many1};

use crate::util::*;

type SolverInput = Vec<Command>;

pub enum Command {
    Down(u32),
    Forward(u32),
    Up(u32),
}

pub fn parse_input(file_bytes: &[u8]) -> SolverInput {
    let down_tag = |i| -> IResult<&[u8],_> { tag(b"down")(i) };
    let forward_tag = |i| -> IResult<&[u8],_> { tag(b"forward")(i) };
    let up_tag = |i| -> IResult<&[u8],_> { tag(b"up")(i) };
    let direction_alt = |i| -> IResult<&[u8],_> { alt((up_tag, forward_tag, down_tag))(i) };
    let line_pair = |i| -> IResult<_, (_, _),_> { separated_pair(direction_alt, tag(b" "), digit1)(i) };
    let command_parse = |i| -> IResult<&[u8], Command> {
        let (rest, (direction, value_str))  = line_pair(i)?;
        if let Some(value) = atoi::atoi(value_str) {
            Ok((rest, match direction[0] {
                b'd' => Command::Down(value),
                b'f' => Command::Forward(value),
                b'u' => Command::Up(value),
                _ => unreachable!(),
            }))
        } else {
            Err(nom::Err::Error(make_error(value_str, error::ErrorKind::Digit)))
        }
    };
    let line_parse = |i| -> IResult<&[u8], _> { terminated(command_parse, tag(b"\n"))(i) };
    let file_parse = |i| -> IResult<&[u8], _> { many1(line_parse)(i) };

    return file_parse(file_bytes).map(move |t| t.1).unwrap();
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut depth = 0;
    let mut distance = 0;
    for command in input {
        match command {
            Command::Down(val) => depth += val,
            Command::Forward(val) => distance += val,
            Command::Up(val) => depth -= val,
        }
    }
    return depth * distance;
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut distance = 0;
    for command in input {
        match command {
            Command::Down(val) => aim += val,
            Command::Forward(val) => {
                distance += val;
                depth += aim * val;
            },
            Command::Up(val) => aim -= val,
        }
    }
    return depth * distance;
}
