fn parse_input(data: &str) -> Vec<Vec<usize>> {
    let mut banks = vec![];
    for line in data.lines() {
        // error here fix tomorrow
        let bank: Vec<usize> = line.chars().map(|s| s.to_digit(10).unwrap() as usize).collect();
        banks.push(bank);
    }

    banks
}

fn select_max_val(batteries: Vec<usize>) -> (usize, usize) {
    let mut first = 0;
    let mut second = 0;
    for battery in batteries {
        println!("battery {}", battery);
        if battery > first {
            first = battery;
        } else if battery > second {
            second = battery;
        }
    }

    (first, second)
}

pub fn solve_part_one(data: &str) -> usize {
    // for tomorrow: find the highest number; pluck it out (teens). repeat for second highest
    let banks = parse_input(data);
    let mut count = 0;
    for bank in banks {
        let (first, second) = select_max_val(bank);

        let mut b = String::new();
        b.push_str(&first.to_string());
        b.push_str(&second.to_string());

        let new_number: usize = b.parse().unwrap();
        count += new_number;
    }
    count
}

pub fn solve_part_two(data: &str) -> usize {
    // Implement the logic for part two of the puzzle here
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let data = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(solve_part_one(data), 357);
    }

    #[test]
    fn test_solve_part_two() {
        let data = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(solve_part_two(data), 0);
    }
}