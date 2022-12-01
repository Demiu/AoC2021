#[macro_export]
macro_rules! _run_day_part_preparsed {
    ($day:literal, $part:literal, $parsed:expr) => {{
        use paste::paste;

        paste! {
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
    }};
}
pub use _run_day_part_preparsed;

#[macro_export]
macro_rules! parse_expect {
    ($from:expr) => {{
        let parsed = parse_input($from);
        parsed.expect("Failed parsing input")
    }};
    ($from:expr, $name:literal) => {{
        let parsed = parse_input($from);
        parsed.expect(concat!("Failed parsing ", $name, " input"))
    }};
    ($day:literal, $from:expr) => {{
        use paste::paste;

        paste! {
            let parsed = [<day $day>]::parse_input($from);
            parsed.expect(concat!(
                "Failed to parse input file for day ",
                $day,
            ))
        }
    }};
}
pub use parse_expect;

#[macro_export]
macro_rules! run_day_p1 {
    ($day:literal) => {
        let input = include_bytes!(concat!("../input/", stringify!($day), "/input.txt",));
        let parsed = parse_expect!($day, input);
        _run_day_part_preparsed!($day, 1, parsed);
    };
}
pub use run_day_p1;

#[macro_export]
macro_rules! run_day {
    ($day:literal) => {
        let input = include_bytes!(concat!("../input/", stringify!($day), "/input.txt",));
        let parsed = parse_expect!($day, input);
        _run_day_part_preparsed!($day, 1, parsed);
        _run_day_part_preparsed!($day, 2, parsed);
    };
}
pub use run_day;

#[cfg(test)]
pub mod test {
    #[macro_export]
    macro_rules! make_test_for_day {
        ($name:ident, $input:expr, $p1:expr, $p2:expr) => {
            use paste::paste;

            paste! {
                #[test]
                fn [< solve_part1_ $name >] () {
                    let parsed = crate::macros::parse_expect!($input);
                    let result = solve_part1(&parsed);
                    assert_eq!(result, $p1);
                }

                #[test]
                fn [< solve_part2_ $name >] () {
                    let parsed = crate::macros::parse_expect!($input);
                    let result = solve_part2(&parsed);
                    assert_eq!(result, $p2);
                }
            }
        };
    }
    pub use make_test_for_day;
}

#[cfg(test)]
pub use test::make_test_for_day;
