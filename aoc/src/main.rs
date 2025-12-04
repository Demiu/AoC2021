use proc::run_year;

mod parse;
mod traits;
mod years;

fn main() {
    run_year!(2021, 25);
    run_year!(2022, 05);
}
