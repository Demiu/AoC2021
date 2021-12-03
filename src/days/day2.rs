use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    error::{self, make_error},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

type SolverInput = Vec<Command>;

pub enum Command {
    Down(u32),
    Forward(u32),
    Up(u32),
}

pub fn parse_input(file_bytes: &[u8]) -> SolverInput {
    let down_literal = |i| tag(b"down")(i);
    let forward_literal = |i| tag(b"forward")(i);
    let up_literal = |i| tag(b"up")(i);
    let direction_alternative = |i| alt((up_literal, forward_literal, down_literal))(i);
    let line_pair = |i| separated_pair(direction_alternative, tag(b" "), digit1)(i);
    let command_parse = |i| {
        let (rest, (direction, value_str)) = line_pair(i)?;
        if let Some(value) = atoi::atoi(value_str) {
            Ok((
                rest,
                match direction[0] {
                    b'd' => Command::Down(value),
                    b'f' => Command::Forward(value),
                    b'u' => Command::Up(value),
                    _ => unreachable!(),
                },
            ))
        } else {
            Err(nom::Err::Error(make_error(
                value_str,
                error::ErrorKind::Digit,
            )))
        }
    };
    let line_parse = |i| terminated(command_parse, tag(b"\n"))(i);
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
            }
            Command::Up(val) => aim -= val,
        }
    }
    return depth * distance;
}
