use std::vec;

fn main() {
    let day1in = include_bytes!("../input/1/input.txt");
    
    let mut numbers = vec![];
    let mut current_number = 0;
    for byte in day1in {
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

    let mut part1solution = 0;
    for i in 1..numbers.len() {
        if numbers[i] > numbers[i - 1] {
            part1solution += 1;
        }
    }
    println!("{}", part1solution);

    let mut part2solution = 0;
    for i in 3..numbers.len() {
        // n[i-2] + n[i-1] + n[i] > n[i-3] + n[i-2] + n[i-1]
        // n[i] > n[i-3]
        if numbers[i] > numbers[i-3] {
            part2solution += 1;
        }
    }
    println!("{}", part2solution);
}
