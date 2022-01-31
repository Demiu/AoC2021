#[macro_export]
macro_rules! _run_day_part_preparsed {
    ($day:literal, $part:literal, $parsed:expr) => {
        {
            use paste::paste;
            
            paste!{
                println!(
                    concat!(
                        "Day ",
                        $day,
                        " Part ",
                        $part,
                        ": {}",
                    ), 
                    [<day $day >]::[< solve_part $part>](&$parsed),
                );
            }
        }
    }
}
pub use _run_day_part_preparsed;

#[macro_export]
macro_rules! run_day_p1 {
    ($day:literal) => {
        {
            use paste::paste;

            paste! {
                let input = include_bytes!(concat!(
                    "../input/",
                    stringify!($day),
                    "/input.txt",
                ));
                let parsed = [<day $day>]::parse_input(input).expect(concat!(
                    "Failed to parse input file for day ",
                    stringify!($day),
                ));
                crate::macros::_run_day_part_preparsed!($day, 1, parsed);
            }
        }
    };
}

#[macro_export]
macro_rules! run_day {
    ($day:literal) => {
        {
            use paste::paste;

            paste! {
                let input = include_bytes!(concat!(
                    "../input/",
                    stringify!($day),
                    "/input.txt",
                ));
                let parsed = [<day $day>]::parse_input(input).expect(concat!(
                    "Failed to parse input file for day ",
                    stringify!($day),
                ));
                crate::macros::_run_day_part_preparsed!($day, 1, parsed);
                crate::macros::_run_day_part_preparsed!($day, 2, parsed);
            }
        }
    };
}

#[cfg(test)]
pub mod test {
    #[macro_export]
    macro_rules! make_test_for_day {
        ($name:ident, $input:expr, $p1:expr, $p2:expr) => {
            use paste::paste;

            paste! {
                #[test]
                fn [< solve_part1_ $name >] () {
                    let parsed = parse_input($input);
                    assert!(parsed.is_ok(), "Failed parsing example input");
                    let result = solve_part1(&parsed.unwrap());
                    assert_eq!(result, $p1);
                }

                #[test]
                fn [< solve_part2_ $name >] () {
                    let parsed = parse_input($input);
                    assert!(parsed.is_ok(), "Failed parsing example input");
                    let result = solve_part2(&parsed.unwrap());
                    assert_eq!(result, $p2);
                }
            }
        };
    }
    pub use make_test_for_day;
}

#[cfg(test)]
pub use test::make_test_for_day;
