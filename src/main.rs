mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

use sky::days;

fn main() {
    let mut args = std::env::args();
    args.next();

    let day = args.next().expect("Provide a parameter specifying which day e.g. 1a means day 1, part A while 4b means day 4, part B").to_ascii_lowercase();
    let day = format!("day{:0>3}", day);

    days!(
        day.as_str(),
        day01,
        day02,
        day03,
        day04,
        day05,
        day06,
        day07,
        day08,
        day09,
    );
}
