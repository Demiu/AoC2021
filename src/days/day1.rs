type SolverInput = Vec<u32>;

pub fn parse_input(file_bytes: &[u8]) -> SolverInput {
    let mut numbers = vec![];
    let mut current_number = 0;
    for byte in file_bytes {
        match byte {
            b'\n' => {
                numbers.push(current_number);
                current_number = 0;
            },
            b'0' ..= b'9' => {
                current_number *= 10;
                current_number += (byte - b'0') as u32;
            },
            _ => panic!()
        }
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
