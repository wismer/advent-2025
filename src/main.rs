use std::env;
use std::fs;

mod solutions;
use solutions::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: cargo run -- <day> <part>");
        eprintln!("Example: cargo run -- day1 p1");
        std::process::exit(1);
    }

    let day_arg = &args[1];
    let part_arg = &args[2];

    let day_num = day_arg.strip_prefix("day").unwrap_or(day_arg);
    let input_file = format!("inputs/{}.txt", day_num);
    let input_data = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Failed to read {}", input_file));

    let result = match (day_arg.as_str(), part_arg.as_str()) {
        ("day1", "p1") => day01::solve_part_one(&input_data),
        ("day1", _) => day01::solve_part_two(&input_data),
        ("day2", "p1") => day02::solve_part_one(&input_data),
        ("day2", _) => day02::solve_part_two(&input_data),
        ("day3", "p1") => day03::solve_part_one(&input_data),
        ("day3", _) => day03::solve_part_two(&input_data),
        ("day4", "p1") => day04::solve_part_one(&input_data),
        ("day4", _) => day04::solve_part_two(&input_data),
        ("day5", "p1") => day05::solve_part_one(&input_data),
        ("day5", _) => day05::solve_part_two(&input_data),
        ("day6", "p1") => day06::solve_part_one(&input_data),
        ("day6", _) => day06::solve_part_two(&input_data),
        ("day7", "p1") => day07::solve_part_one(&input_data),
        ("day7", _) => day07::solve_part_two(&input_data),
        ("day8", "p1") => day08::solve_part_one(&input_data),
        ("day8", _) => day08::solve_part_two(&input_data),
        ("day9", "p1") => day09::solve_part_one(&input_data),
        ("day9", _) => day09::solve_part_two(&input_data),
        ("day10", "p1") => day10::solve_part_one(&input_data),
        ("day10", _) => day10::solve_part_two(&input_data),
        ("day11", "p1") => day11::solve_part_one(&input_data),
        ("day11", _) => day11::solve_part_two(&input_data),
        ("day12", "p1") => day12::solve_part_one(&input_data),
        ("day12", _) => day12::solve_part_two(&input_data),
        _ => {
            eprintln!("Unknown day or part: {} {}", day_arg, part_arg);
            std::process::exit(1);
        }
    };

    println!("{}", result);
}