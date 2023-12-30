mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

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
        day10,
        day11,
        day12,
        day13,
        day14,
        day15,
        day16,
        day17,
        day18,
        day19,
        day20,
        day21,
        day22,
        day23,
        day24,
        day25,
    );
}
