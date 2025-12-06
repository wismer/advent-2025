use core::fmt;

#[derive(Debug)]
struct Point(usize, usize, bool);
struct PrintDeptMap {
    pub grid: Vec<Vec<Point>>,
}

impl fmt::Display for PrintDeptMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map: String = self.grid.iter().map(|f| {
            let mut row = f.iter().map(|p| {
                if p.2 {
                    "@"
                } else {
                    "."
                }
            }).collect::<Vec<_>>().join("");
            row.push('\n');
            row
        }).collect();
        write!(f, "{}", map)
    }
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

impl PrintDeptMap {
    fn remove_roll(&mut self, x: usize, y: usize) {
        let mut row = self.grid.get_mut(x).unwrap();
        match row.get_mut(y) {
            Some(pt) => {
                pt.2 = false;
            },
            None => {}
        }
    }

    fn count_moveable_rolls(&self) -> Vec<(usize, usize)> {
        let mut moveable_rolls: Vec<(usize, usize)> = vec![];
        for row in &self.grid {
            for pt in row {
                let mut rolls = 0;
                if pt.2 {
                    let adj_pts = self.get_adj_pts(pt.0, pt.1);
                    for adj_pt in adj_pts {
                        match adj_pt {
                            Some(Point(x, y, true)) => {
                                rolls += 1
                            },
                            _ => {}
                        }
                    }
                    println!("pt: {pt:?}, rolls: {rolls}");
                    if rolls < 4 {
                        moveable_rolls.push((pt.0, pt.1));
                    }
                }
                
            }
        }

        moveable_rolls
    }

    fn get_adj_pts(&self, x: usize, y: usize) -> [Option<&Point>; 8] {
        let dimension = self.grid.len() as isize;
        ADJACENT_POINTS.map(|pt| {
            let (xp, yp) = (pt.0 + x as isize, pt.1 + y as isize);
                // println!("xp {xp} yp {yp} pt: {pt:?} x: {x} y: {y} s:{}", pt.1 + y as isize);
            if xp > dimension || yp > dimension || xp < 0 || yp < 0 {
                None
            } else {
                match self.grid.get((x as isize + pt.0) as usize) {
                    Some(row) => row.get((y as isize + pt.1) as usize),
                    None => None
                }
            }
        })
    }
}


fn parse_input(data: &str) -> PrintDeptMap {
    let mut map = PrintDeptMap { grid: vec![] };
    for (x, line) in data.lines().enumerate() {
        let mut pts = vec![];
        for (y, c) in line.chars().enumerate() {
            let pt = Point(x, y, c == '@');

            pts.push(pt);
        }
        map.grid.push(pts);
    }

    map
}

pub fn solve_part_one(data: &str) -> usize {
    let map = parse_input(data);
    map.count_moveable_rolls().len()
}

pub fn solve_part_two(data: &str) -> usize {
    let mut map = parse_input(data);
    let mut removed_rolls = 0;
    loop {
        let moveable_rolls = map.count_moveable_rolls();

        if moveable_rolls.is_empty() {
            break;
        }
        removed_rolls += moveable_rolls.len();        
        for roll in moveable_rolls {
            map.remove_roll(roll.0, roll.1);
        }
    }
    
    // Return the count of remaining rolls
    removed_rolls
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
        assert_eq!(solve_part_two(SAMPLE), 43);
    }
}