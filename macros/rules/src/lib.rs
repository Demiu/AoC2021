#[macro_export]
macro_rules! _run_day_part_preparsed {
    ($day:literal, $part:literal, $parsed:expr) => {{
        use paste::paste;

        paste! {
            let result = [<day $day >]::[< solve_part $part>](&$parsed);
            let mut result_str = format!("{}", result);
            if result_str.contains("\n") {
                result_str.insert(0, '\n');
            }
            println!(
                concat!(
                    "Day ",
                    $day,
                    " Part ",
                    $part,
                    ": {}",
                ),
                result_str,
            );
        }
    }};
}

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

#[macro_export]
macro_rules! run_day_p1 {
    ($year:literal, $day:literal) => {
        let input = include_bytes!(concat!(
            "../input/",
            stringify!($year),
            "/",
            stringify!($day),
            "/input.txt",
        ));
        let parsed = parse_expect!($day, input);
        _run_day_part_preparsed!($day, 1, parsed);
    };
}

#[macro_export]
macro_rules! run_day {
    ($year:literal, $day:literal) => {
        let input = include_bytes!(concat!(
            "../input/",
            stringify!($year),
            "/",
            stringify!($day),
            "/input.txt",
        ));
        let parsed = parse_expect!($day, input);
        _run_day_part_preparsed!($day, 1, parsed);
        _run_day_part_preparsed!($day, 2, parsed);
    };
}

#[macro_export]
macro_rules! make_test_for_day {
    ($name:ident, $input:expr, $p1:expr, $p2:expr) => {
        use paste::paste;

        paste! {
            #[test]
            fn [< solve_part1_ $name >] () {
                let parsed = rules::parse_expect!($input);
                let result = solve_part1(&parsed);
                assert_eq!(result, $p1);
            }

            #[test]
            fn [< solve_part2_ $name >] () {
                let parsed = rules::parse_expect!($input);
                let result = solve_part2(&parsed);
                assert_eq!(result, $p2);
            }
        }
    };
}
