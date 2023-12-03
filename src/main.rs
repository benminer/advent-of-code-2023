use std::env;

mod aoc;
mod day1;

use aoc::day::AdventOfCodeDay;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut day = 1;
    let mut part = 1;

    for i in 0..args.len() {
        if args[i] == "--day" {
            day = args[i + 1].parse::<u8>().unwrap();
        }

        if args[i] == "--part" {
            part = args[i + 1].parse::<u8>().unwrap();
        }
    }

    let aoc_day = AdventOfCodeDay::new(day);

    match day {
        1 => {
            match part {
                1 => day1::part1(&aoc_day),
                2 => day1::part2(&aoc_day),
                _ => println!("Part {} not implemented for day {}", part, day),
            }
        }
        _ => println!("Day {} not implemented", day),
    }
}
