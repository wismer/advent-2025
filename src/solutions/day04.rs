
struct Point(usize, usize, bool);
struct PrintDeptMap {
    pub grid: Vec<Vec<Point>>
}

const ADJACENT_POINTS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1)
];


fn parse_input(data: &str) -> PrintDeptMap {
    let mut map = PrintDeptMap { grid: vec![] };
    for (x, line) in data.lines().enumerate() {
        let mut pts = vec![];
        for (y, c) in line.chars().enumerate() {
            pts.push(Point(x, y, c == '@'));
        }
        map.grid.push(pts);
    }

    map
}

pub fn solve_part_one(data: &str) -> usize {
    let map = parse_input(data);
    
}

pub fn solve_part_two(data: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &'static str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(SAMPLE), 13);
    }

    #[test]
    fn test_solve_part_two() {
        let data = "";
        assert_eq!(solve_part_two(SAMPLE), 0);
    }
}