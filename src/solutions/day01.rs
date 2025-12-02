// This file contains the solution for Day 01 of the Advent of Code challenge, including functions to read input and compute results.

fn parse_line(line: &str) -> (char, usize) {
    let (turn, dist) = line.split_at(1);
    let distance: usize = dist.parse().unwrap();
    (turn.chars().next().unwrap(), distance)
}

pub fn solve_part_one(data: &str) -> usize {
    // 0 - 99 valid locations
    let instructions = data.lines();
    let mut count = 0;
    let mut position = 50;

    for line in instructions {
        let (dir, n) = parse_line(line);
        let modulo = n % 100;
        if dir == 'R' {
            position = (modulo + position) % 100;
        } else {
            let diff = if position > modulo {
                position - modulo
            } else {
                modulo - position
            };
            position = diff;
        }

        if position == 0 {
            count += 1;
        }
    }

    count
}

pub fn solve_part_two(data: &str) -> usize {
    let instructions = data.lines();
    let mut count = 0;
    let mut position = 50;
    // let's just be dumb for a moment
    for line in instructions {
        let (dir, n) = parse_line(line);
        let mut movements = n;
        while movements != 0 {
            if dir == 'R' {
                if position == 99 {
                    position = 0;
                } else {
                    position += 1;
                }
            } else {
                if position == 0 {
                    position = 99;
                } else {
                    position -= 1;
                }
            }

            if position == 0 {
                count += 1;
            }
            movements -= 1;
        }
    }

    count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let data = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let expected = 3;
        let actual = solve_part_one(data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve_part_two() {
        let data = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let expected = 6;
        let actual = solve_part_two(data);
        assert_eq!(expected, actual);
    }
}