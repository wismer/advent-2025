// This file demonstrates how to run a specific day's solution for the Advent of Code challenge.

use advent_of_code_rust::solutions::{day01, day02};

fn main() {
    let day = std::env::args().nth(1).expect("Please provide a day number (e.g., 1 or 2)");
    
    match day.as_str() {
        "1" => {
            let input = day01::read_input("inputs/day01.txt");
            let result = day01::solve(&input);
            println!("Day 01 Result: {}", result);
        },
        "2" => {
            let input = day02::read_input("inputs/day02.txt");
            let result = day02::solve(&input);
            println!("Day 02 Result: {}", result);
        },
        _ => {
            eprintln!("Invalid day number. Please provide 1 or 2.");
        }
    }
}