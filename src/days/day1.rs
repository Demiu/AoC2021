use crate::util::{scan_ascii_to_u32, skip_ascii_whitespace};

type SolverInput = Vec<u32>;

pub fn parse_input(file_bytes: &[u8]) -> SolverInput {
    let mut numbers = vec![];
    let mut subslice = file_bytes;
    while subslice.len() > 0 {
        let (number, new_subslice) = scan_ascii_to_u32(subslice);
        numbers.push(number);
        subslice = skip_ascii_whitespace(new_subslice);
    }
    return numbers;
}

pub fn solve_part1(input: &SolverInput) -> u32 {
    let mut solution = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            solution += 1;
        }
    }
    return solution;
}

pub fn solve_part2(input: &SolverInput) -> u32 {
    let mut solution = 0;
    for i in 3..input.len() {
        // n[i-2] + n[i-1] + n[i] > n[i-3] + n[i-2] + n[i-1]
        // n[i] > n[i-3]
        if input[i] > input[i - 3] {
            solution += 1;
        }
    }
    return solution;
}
