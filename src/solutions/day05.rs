use std::{cmp::Ordering, ops::RangeInclusive, thread::current};

fn parse_input(data: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let (raw_ranges, ids) = data.split_once("\n\n").unwrap();
    (
        raw_ranges.lines().map(|r| {
            let ns: Vec<&str> = r.split("-").collect();
            let start = ns[0].parse().unwrap();
            let end = ns[1].parse().unwrap();
            start..=end
        }).collect(),
        ids.lines().map(|id| id.parse().unwrap()).collect()
    )
}

pub fn solve_part_one(data: &str) -> usize {
    let (ranges, ids) = parse_input(data);
    let mut fresh_count = 0;
    for id in ids {
        let contains_id = ranges.iter().any(|r| r.contains(&id));
        if contains_id {
            fresh_count += 1;
        }
    }

    fresh_count
}

pub fn solve_part_two(data: &str) -> usize {
    let (ranges, _ids) = parse_input(data);
    let mut t_ranges: Vec<(usize, usize)> = ranges.iter().map(|r| {
        (*r.start(), *r.end())
    }).collect();

    // Sort by start, then by end descending
    t_ranges.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            b.1.cmp(&a.1)
        }
    });

    let mut merged: Vec<(usize, usize)> = vec![];
    
    for (start, end) in t_ranges {
        match merged.last_mut() {
            Some((prev_start, prev_end)) => {
                // If current overlaps or touches previous, merge
                if start <= *prev_end + 1 {
                    *prev_end = (*prev_end).max(end);
                } else {
                    // No overlap, add as new range
                    merged.push((start, end));
                }
            },
            None => {
                merged.push((start, end));
            }
        }
    }

    merged.iter().map(|(start, end)| end - start + 1).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &'static str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(SAMPLE_DATA), 3);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(SAMPLE_DATA), 14);
    }
}