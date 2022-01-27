use anyhow::{bail, Result};

type ParserOutput = Vec<Command>;
type SolverInput = [Command];

#[derive(Debug, PartialEq)]
pub enum Command {
    Down(u8),
    Forward(u8),
    Up(u8),
}

pub fn parse_input(file: &[u8]) -> Result<ParserOutput> {
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
    depth * distance
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
    depth * distance
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = concat!(
        "forward 5\n",
        "down 5\n",
        "forward 8\n",
        "up 3\n",
        "down 8\n",
        "forward 2\n",
    )
    .as_bytes();

    #[test]
    fn parse_example() {
        use super::Command::*;

        let parsed = parse_input(EXAMPLE);
        assert!(parsed.is_ok(), "Failed parsing example input");
        assert_eq!(
            parsed.unwrap(),
            [Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)]
        )
    }

    crate::macros::make_test_for_day!(example, EXAMPLE, 150, 900);
}
