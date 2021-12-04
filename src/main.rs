mod days;
mod parse;

use std::time::Instant;

use days::*;

fn main() {
    let day1in = include_bytes!("../input/1/input.txt");
    let day1parsed = day1::parse_input(day1in).expect("Failed to parse input file for day 1");
    println!("Day 1 Part 1: {}", day1::solve_part1(&day1parsed));
    println!("Day 1 Part 2: {}", day1::solve_part2(&day1parsed));

    let day2in = include_bytes!("../input/2/input.txt");
    let day2parsed = day2::parse_input(day2in).expect("Failed to parse input file for day 2");
    println!("Day 2 Part 1: {}", day2::solve_part1(&day2parsed));
    println!("Day 2 Part 2: {}", day2::solve_part2(&day2parsed));

    let day3in = include_bytes!("../input/3/input.txt");
    let day3parsed = day3::parse_input(day3in).expect("Failed to parse input file for day 3");
    println!("Day 3 Part 1: {}", day3::solve_part1(&day3parsed));
    println!("Day 3 Part 2: {}", day3::solve_part2(&day3parsed));

    let day4in = include_bytes!("../input/4/input.txt");
    let day4parsed = day4::parse_input(day4in).expect("Failed to parse input file for day 4");
    println!("Day 4 Part 1: {}", day4::solve_part1(&day4parsed));
    println!("Day 4 Part 2: {}", day4::solve_part2(&day4parsed));
}
