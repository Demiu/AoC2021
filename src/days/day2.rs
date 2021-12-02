use crate::util::*;

type SolverInput = Vec<Command>;

pub enum Command {
    Down(u32),
    Forward(u32),
    Up(u32),
}

pub fn parse_input(file_bytes: &[u8]) -> SolverInput {
    let mut commands = vec![];

    let mut subslice = file_bytes;
    while subslice.len() > 0 {
        let character = subslice[0];
        subslice = skip_ascii_whitespace(skip_to_ascii_whitespace(subslice));
        let (magnitude, new_subslice) = scan_ascii_to_u32(subslice);

        commands.push(match character {
            b'd' => Command::Down(magnitude),
            b'f' => Command::Forward(magnitude),
            b'u' => Command::Up(magnitude),
            _ => panic!(),
        });

        subslice = skip_ascii_whitespace(new_subslice);
    }

    return commands;
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
