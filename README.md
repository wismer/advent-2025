# Advent of Code Rust Solutions

This project contains solutions for the Advent of Code challenges implemented in Rust. Each day's solution is organized into separate modules for clarity and maintainability.

## Project Structure

- `src/main.rs`: Entry point of the application.
- `src/lib.rs`: Library file for shared functionality.
- `src/solutions/`: Contains individual solution modules for each day.
  - `day01.rs`: Solution for Day 01.
  - `day02.rs`: Solution for Day 02.
- `src/utils/`: Utility functions for input parsing and common calculations.
- `inputs/`: Input data files for each day's challenge.
- `examples/`: Example scripts demonstrating how to run specific solutions.
- `benches/`: Benchmarking scripts for performance testing.
- `tests/`: Integration tests to ensure solutions work as expected.

## Setup Instructions

1. Ensure you have Rust installed on your machine. You can install it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone this repository:
   ```
   git clone <repository-url>
   ```
3. Navigate to the project directory:
   ```
   cd advent-of-code-rust
   ```
4. Run the solutions using:
   ```
   cargo run
   ```

## Usage Examples

To run a specific day's solution, you can modify the `src/main.rs` file to call the desired function from the corresponding module in `src/solutions/`.

## Contributing

Feel free to contribute by adding new solutions or improving existing ones. Make sure to follow the project's structure and coding conventions.

Happy coding!