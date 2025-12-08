#[derive(Debug)]
enum Operation {
    Mult,
    Sub,
    Add,
    Div
}

fn parse_input(data: &str) -> (Vec<Vec<usize>>, Vec<Operation>) {
    let mut operations: Vec<Operation> = vec![];
    let mut parsed_nums: Vec<Vec<usize>> = vec![];
    for (idx, line) in data.lines().rev().enumerate() {
        let split_text: Vec<&str> = line.split_ascii_whitespace().collect();
        if idx == 0 {
            for raw_op in split_text {
                let op = match raw_op {
                    "*" => Operation::Mult,
                    "+" => Operation::Add,
                    "/" => Operation::Div,
                    "-" => Operation::Sub,
                    _ => panic!("shouldnt happen")
                };
                operations.push(op);
            }
        } else {

            let parsed_num_set: Vec<usize> = split_text.iter().map(|rn| rn.parse().unwrap()).collect();
            parsed_nums.push(parsed_num_set);
        }
    }

    (parsed_nums, operations)
}

fn get_nested(from: &Vec<Vec<usize>>, coord: (usize, usize)) -> Option<&usize> {
    match from.get(coord.0) {
        Some(row) => row.get(coord.1),
        None => None
    }
}

pub fn solve_part_one(data: &str) -> usize {
    let (nums, ops) = parse_input(data);
    let max_rows = nums.len();
    let mut total = 0;
    for (x, op) in ops.iter().enumerate() {
        let mut col_nums: Vec<usize> = vec![];
        for y in 0..max_rows {
            let n = get_nested(&nums, (y, x)).unwrap();
            col_nums.push(*n);
        }

        total += match op {
            Operation::Add => col_nums.iter().sum(),
            Operation::Mult => col_nums.iter().product(),
            _ => 0
        };
    }

    total
}

fn char_to_op(c: &char) -> Option<Operation> {
    match c {
        '*' => Some(Operation::Mult),
        '+' => Some(Operation::Add),
        _ => None
    }
}

pub fn solve_part_two(data: &str) -> usize {
    let mut total = 0;
    let lines: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    let mut idx = 0;
    let mut op: Operation = Operation::Add;
    let max = lines.len();

    let mut nums_for_operation: Vec<usize> = vec![];
    while idx < lines[0].len() {
        let mut nums = String::new();
        // get the characters from the column and smush them into a string
        for i in 0..(max - 1) {
            if lines[i][idx] != ' ' {
                nums.push(lines[i][idx]);
            }
        }

        op = match char_to_op(&lines[max - 1][idx]) {
            Some(o) => o,
            None => op,
        };

        

        // if parsing fails, then we move onto the next operation section
        match nums.parse::<usize>() {
            Err(_) => {
                idx += 1;
                println!("{nums_for_operation:?} {idx:?}");
                match op {
                    Operation::Add => total += nums_for_operation.iter().sum::<usize>(),
                    Operation::Mult => total += nums_for_operation.iter().product::<usize>(),
                    _ => {}
                }
                nums_for_operation.clear();
                continue;
            },
            // if it doesn't, we put it into a bucket to be used later
            Ok(n) => {
                nums_for_operation.push(n);
            }
        }
        
        idx += 1;
    }

    // do it one last time
    match op {
        Operation::Add => total + nums_for_operation.iter().sum::<usize>(),
        Operation::Mult => total + nums_for_operation.iter().product::<usize>(),
        _ => total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &'static str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(SAMPLE_DATA), 4277556);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(SAMPLE_DATA), 3263827);
    }
}