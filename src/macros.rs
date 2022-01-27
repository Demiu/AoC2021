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
