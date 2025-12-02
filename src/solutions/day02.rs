use std::ops::RangeInclusive;


// This file contains the solution for Day 02 of the Advent of Code challenge. 
// It includes functions to read input and compute results.
fn parse_input(input: &str) -> Vec<RangeInclusive<usize>> {
    let raw_ranges = input.split(",");
    let mut ranges: Vec<RangeInclusive<usize>> = vec![];
    for raw_range in raw_ranges {
        let vals: Vec<usize> = raw_range.split("-").map(|s| {
            let n = s.parse();

            match n {
                Ok(n) => n,
                Err(e) => {
                    0
                }
            }
        }).collect();


        ranges.push(vals[0]..=vals[1]);
    }

    ranges
}

pub fn solve_part_one(input: &str) -> usize {
    let ranges = parse_input(input);
    let mut tally = 0usize;
    for range in ranges {
        for i in range {
            // convert num to string
            let s = i.to_string();
            let s_len = s.len();
            if s_len % 2 == 0 {
                let (first, second) = s.split_at(s_len / 2);
                // println!("first {}, second {}", first, second);
                if first == second {
                    tally += i;
                }
            }
        }
    }

    tally
}

pub fn solve_part_two(input: &str) -> usize {
    let ranges = parse_input(input);
    let mut tally = 0;
    for range in ranges {
        for i in range {
            let s = i.to_string();
            let s_len = s.len();

            for n in 1..=(s_len / 2) {
                // let c = s.get(0..=n).unwrap();
                let (first, rest) = s.split_at(n);
                let pattern_matched = rest.split_inclusive(first).all(|subs| subs == first);

                if pattern_matched {
                    tally += i;
                    break;
                }
            }
        }
    }

    tally
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let expected = 1227775554usize;
        let actual = solve_part_one(data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve_part_two() {
        let data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let expected = 4174379265;
        let actual = solve_part_two(data);
        assert_eq!(expected, actual);
    }
}
