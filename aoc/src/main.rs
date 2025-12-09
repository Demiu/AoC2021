use chrono::Datelike;
use proc::run_year;
use rules::parse_expect;

mod parse;
mod traits;
mod years;

fn main() {
    match chrono::Local::now().year() {
        2021 => run_year!(2021, 25),
        2022 => run_year!(2022, 05),
        2025 => run_year!(2025, 09),
        _ => println!("Either AoC didn't start this year or you're lazy"),
    }
}
