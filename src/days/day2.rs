use anyhow::{bail, Result};

type SolverInput = Vec<Command>;

pub enum Command {
    Down(u8),
    Forward(u8),
    Up(u8),
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    let mut parsed = vec![];
    let mut index = 0;
    while let Some(ch) = file.get(index) {
        let command = match *ch {
            b'd' => {
                index += 7;
                Command::Down(file[index - 2] - b'0')
            }
            b'f' => {
                index += 10;
                Command::Forward(file[index - 2] - b'0')
            }
            b'u' => {
                index += 5;
                Command::Up(file[index - 2] - b'0')
            }
            _ => bail!("Starting character doesn't match any possible option"),
        };
        parsed.push(command);
    }
    Ok(parsed)
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut depth = 0;
    let mut distance = 0;
    for command in input {
        match command {
            Command::Down(val) => depth += *val as u32,
            Command::Forward(val) => distance += *val as u32,
            Command::Up(val) => depth -= *val as u32,
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
            Command::Down(val) => aim += *val as u32,
            Command::Forward(val) => {
                distance += *val as u32;
                depth += aim * (*val as u32);
            }
            Command::Up(val) => aim -= *val as u32,
        }
    }
    return depth * distance;
}
