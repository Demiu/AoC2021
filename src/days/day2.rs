use crate::util::*;

type SolverInput = Vec<Command>;

pub enum Direction {
    Down,
    Forward,
    Up,
}
type Command = (Direction, u32);

pub fn parse_input(file_bytes: &[u8]) -> SolverInput {
    let mut commands = vec![];

    let mut subslice = file_bytes;
    while subslice.len() > 0 {
        let character = subslice[0];
        let direction = match character {
            b'd' => Direction::Down,
            b'f' => Direction::Forward,
            b'u' => Direction::Up,
            _ => panic!(),
        };

        subslice = skip_ascii_whitespace(skip_to_ascii_whitespace(subslice));
        let (magnitude, new_subslice) = scan_ascii_to_u32(subslice);

        commands.push((direction, magnitude));

        subslice = skip_ascii_whitespace(new_subslice);
    }

    return commands;
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut depth = 0;
    let mut distance = 0;
    for command in input {
        match command.0 {
            Direction::Down => depth += command.1,
            Direction::Forward => distance += command.1,
            Direction::Up => depth -= command.1,
        }
    }
    return depth * distance;
}
