mod days;
mod parse;

use days::*;

fn main() {
    let day1in = include_bytes!("../input/1/input.txt");
    let day1parsed = day01::parse_input(day1in).expect("Failed to parse input file for day 1");
    println!("Day 1 Part 1: {}", day01::solve_part1(&day1parsed));
    println!("Day 1 Part 2: {}", day01::solve_part2(&day1parsed));

    let day2in = include_bytes!("../input/2/input.txt");
    let day2parsed = day02::parse_input(day2in).expect("Failed to parse input file for day 2");
    println!("Day 2 Part 1: {}", day02::solve_part1(&day2parsed));
    println!("Day 2 Part 2: {}", day02::solve_part2(&day2parsed));

    let day3in = include_bytes!("../input/3/input.txt");
    let day3parsed = day03::parse_input(day3in).expect("Failed to parse input file for day 3");
    println!("Day 3 Part 1: {}", day03::solve_part1(&day3parsed));
    println!("Day 3 Part 2: {}", day03::solve_part2(&day3parsed));

    let day4in = include_bytes!("../input/4/input.txt");
    let day4parsed = day04::parse_input(day4in).expect("Failed to parse input file for day 4");
    println!("Day 4 Part 1: {}", day04::solve_part1(&day4parsed));
    println!("Day 4 Part 2: {}", day04::solve_part2(&day4parsed));

    let day5in = include_bytes!("../input/5/input.txt");
    let day5parsed = day05::parse_input(day5in).expect("Failed to parse input file for day 5");
    println!("Day 5 Part 1: {}", day05::solve_part1(&day5parsed));
    println!("Day 5 Part 2: {}", day05::solve_part2(&day5parsed));

    let day6in = include_bytes!("../input/6/input.txt");
    let day6parsed = day06::parse_input(day6in).expect("Failed to parse input file for day 6");
    println!("Day 6 Part 1: {}", day06::solve_part1(&day6parsed));
    println!("Day 6 Part 2: {}", day06::solve_part2(&day6parsed));

    let day7in = include_bytes!("../input/7/input.txt");
    let day7parsed = day07::parse_input(day7in).expect("Failed to parse input file for day 7");
    println!("Day 7 Part 1: {}", day07::solve_part1(&day7parsed));
    println!("Day 7 Part 2: {}", day07::solve_part2(&day7parsed));

    let day8in = include_bytes!("../input/8/input.txt");
    let day8parsed = day08::parse_input(day8in).expect("Failed to parse input file for day 8");
    println!("Day 8 Part 1: {}", day08::solve_part1(&day8parsed));
    println!("Day 8 Part 2: {}", day08::solve_part2(&day8parsed));

    let day9in = include_bytes!("../input/9/input.txt");
    let day9parsed = day09::parse_input(day9in).expect("Failed to parse input file for day 9");
    println!("Day 9 Part 1: {}", day09::solve_part1(&day9parsed));
    println!("Day 9 Part 2: {}", day09::solve_part2(&day9parsed));

    let day10in = include_bytes!("../input/10/input.txt");
    let day10parsed = day10::parse_input(day10in).expect("Failed to parse input file for day 10");
    println!("Day 10 Part 1: {}", day10::solve_part1(&day10parsed));
    println!("Day 10 Part 2: {}", day10::solve_part2(&day10parsed));

    let day11in = include_bytes!("../input/11/input.txt");
    let day11parsed = day11::parse_input(day11in).expect("Failed to parse input file for day 11");
    println!("Day 11 Part 1: {}", day11::solve_part1(&day11parsed));
    println!("Day 11 Part 2: {}", day11::solve_part2(&day11parsed));

    let day12in = include_bytes!("../input/12/input.txt");
    let day12parsed = day12::parse_input(day12in).expect("Failed to parse input file for day 12");
    println!("Day 12 Part 1: {}", day12::solve_part1(&day12parsed));
    println!("Day 12 Part 2: {}", day12::solve_part2(&day12parsed));

    let day13in = include_bytes!("../input/13/input.txt");
    let day13parsed = day13::parse_input(day13in).expect("Failed to parse input file for day 13");
    println!("Day 13 Part 1: {}", day13::solve_part1(&day13parsed));
    println!("Day 13 Part 2: \n{}", day13::solve_part2(&day13parsed));

    let day14in = include_bytes!("../input/14/input.txt");
    let day14parsed = day14::parse_input(day14in).expect("Failed to parse input file for day 14");
    println!("Day 14 Part 1: {}", day14::solve_part1(&day14parsed));
    println!("Day 14 Part 2: {}", day14::solve_part2(&day14parsed));

    let day15in = include_bytes!("../input/15/input.txt");
    let day15parsed = day15::parse_input(day15in).expect("Failed to parse input file for day 15");
    println!("Day 15 Part 1: {}", day15::solve_part1(&day15parsed));
    println!("Day 15 Part 2: {}", day15::solve_part2(&day15parsed));

    let day16in = include_bytes!("../input/16/input.txt");
    let day16parsed = day16::parse_input(day16in).expect("Failed to parse input file for day 16");
    println!("Day 16 Part 1: {}", day16::solve_part1(&day16parsed));
    println!("Day 16 Part 2: {}", day16::solve_part2(&day16parsed));

    let day17in = include_bytes!("../input/17/input.txt");
    let day17parsed = day17::parse_input(day17in).expect("Failed to parse input file for day 17");
    println!("Day 17 Part 1: {}", day17::solve_part1(&day17parsed));
    println!("Day 17 Part 2: {}", day17::solve_part2(&day17parsed));

    let day18in = include_bytes!("../input/18/input.txt");
    let day18parsed = day18::parse_input(day18in).expect("Failed to parse input file for day 18");
    println!("Day 18 Part 1: {}", day18::solve_part1(&day18parsed));
    println!("Day 18 Part 2: {}", day18::solve_part2(&day18parsed));

    let day19in = include_bytes!("../input/19/input.txt");
    let day19parsed = day19::parse_input(day19in).expect("Failed to parse input file for day 19");
    println!("Day 19 Part 1: {}", day19::solve_part1(&day19parsed));
    println!("Day 19 Part 2: {}", day19::solve_part2(&day19parsed));

    let day20in = include_bytes!("../input/20/input.txt");
    let day20parsed = day20::parse_input(day20in).expect("Failed to parse input file for day 20");
}
