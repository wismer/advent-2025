use std::env;
use std::fs;

mod solutions;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: cargo run -- <day> <part>");
        eprintln!("Example: cargo run -- day1 p1");
        std::process::exit(1);
    }

    let day_arg = &args[1];
    let part_arg = &args[2];

    let input_file = format!("inputs/{}.txt", day_arg.as_str());
    let input_data = fs::read_to_string(&input_file)?;

    let result = match (day_arg.as_str(), part_arg.as_str()) {
        ("day1", "p1") => solutions::day01::solve_part_one(&input_data),
        ("day1", _) => solutions::day01::solve_part_two(&input_data),
        ("day2", "p1") => solutions::day02::solve_part_one(&input_data),
        ("day2", _) => solutions::day02::solve_part_two(&input_data),
        ("day3", "p1") => solutions::day03::solve_part_one(&input_data),
        ("day3", _) => solutions::day03::solve_part_two(&input_data),
        ("day4", "p1") => solutions::day04::solve_part_one(&input_data),
        ("day4", _) => solutions::day04::solve_part_two(&input_data),
        ("day5", "p1") => solutions::day05::solve_part_one(&input_data),
        ("day5", _) => solutions::day05::solve_part_two(&input_data),
        ("day6", "p1") => solutions::day06::solve_part_one(&input_data),
        ("day6", _) => solutions::day06::solve_part_two(&input_data),
        ("day7", "p1") => solutions::day07::solve_part_one(&input_data),
        ("day7", _) => solutions::day07::solve_part_two(&input_data),
        ("day8", "p1") => solutions::day08::solve_part_one(&input_data),
        ("day8", _) => solutions::day08::solve_part_two(&input_data),
        ("day9", "p1") => solutions::day09::solve_part_one(&input_data),
        ("day9", _) => solutions::day09::solve_part_two(&input_data),
        ("day10", "p1") => solutions::day10::solve_part_one(&input_data),
        ("day10", _) => solutions::day10::solve_part_two(&input_data),
        ("day11", "p1") => solutions::day11::solve_part_one(&input_data),
        ("day11", _) => solutions::day11::solve_part_two(&input_data),
        ("day12", "p1") => solutions::day12::solve_part_one(&input_data),
        ("day12", _) => solutions::day12::solve_part_two(&input_data),
        _ => {
            eprintln!("Unknown day or part: {} {}", day_arg, part_arg);
            std::process::exit(1);
        }
    };

    println!("{}", result);

    Ok(())
}