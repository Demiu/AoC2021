mod days;
mod parse;

use days::*;

fn main() {
    let day1in = include_bytes!("../input/1/input.txt");
    let numbers = day1::parse_input(day1in).expect("Failed to parse input file for day 1");
    println!("Day 1 Part 1: {}", day1::solve_part1(&numbers));
    println!("Day 1 Part 2: {}", day1::solve_part2(&numbers));

    let day2in = include_bytes!("../input/2/input.txt");
    let commands = day2::parse_input(day2in).expect("Failed to parse input file for day 2");
    println!("Day 2 Part 1: {}", day2::solve_part1(&commands));
    println!("Day 2 Part 2: {}", day2::solve_part2(&commands));

    let day3in = include_bytes!("../input/3/input.txt");
    let input = day3::parse_input(day3in).expect("Failed to parse input file for day 3");
    println!("Day 3 Part 1: {}", day3::solve_part1(&input));
}
