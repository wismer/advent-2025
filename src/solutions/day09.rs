use std::{collections::HashSet, fmt::{Debug, Display}, usize};


struct Grid(HashSet<(usize, usize)>, HashSet<(usize, usize)>);


impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut max_x_bound = 0;
        let mut max_y_bound = 0;
        let mut min_x_bound = usize::MAX;
        let mut min_y_bound = usize::MAX;
        for (x, y) in self.0.iter() {
            if *x > max_x_bound {
                max_x_bound = *x;
            } 
            
            if *x < min_x_bound {
                min_x_bound = *x;
            }

            if *y > max_y_bound {
                max_y_bound = *y;
            }

            if *y < min_y_bound {
                min_y_bound = *y;
            }
        }

        let mut grid: String = String::new();
        // println!("{max_x_bound} {max_y_bound}");
        for x in (min_x_bound)..=(max_x_bound + 2) {
            for y in (min_y_bound)..=(max_y_bound + 2) {
                if self.0.contains(&(x, y)) {
                    grid.push('#');
                } else {
                    grid.push('.');
                }
            }
            grid.push('\n');
        }


        write!(f, "{}", grid)
    }
}

fn area(coord_a: &(usize, usize), coord_b: &(usize, usize)) -> Option<usize> {
    let height = coord_a.0.abs_diff(coord_b.0) + 1;
    let width = coord_a.1.abs_diff(coord_b.1) + 1;

    Some(height * width)
}

fn map_green_tiles(red_tiles: &Vec<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut green_tiles = HashSet::new();
    let rt: Vec<&(usize, usize)> = red_tiles.iter().collect();
    for slice in rt.windows(2) {
        let a = slice[0];
        let b = slice[1];
        // same row, but runs left to right
        let path: Vec<(usize, usize)> = if a.0 == b.0 && a.1 < b.1 {
            ((a.1 + 1)..b.1).map(|n| (a.0, n)).collect()
        } else if a.0 == b.0 && a.1 > b.1 {
            (b.1..(a.1 - 1)).map(|n| (a.0, n + 1)).collect()
        } else if a.1 == b.1 && a.0 < b.0 {
            ((a.0 + 1)..b.0).map(|n| (n, a.1)).collect()
        } else {
            (b.0..(a.0 - 1)).map(|n| (n + 1, a.1)).collect()
        };

        for coord in path {
            green_tiles.insert(coord);
        }
    }


    green_tiles
}

fn parse_input(data: &str) -> Grid {
    let mut hs: HashSet<(usize, usize)> = HashSet::new();
    let mut rt: Vec<(usize, usize)> = vec![];
    for line in data.lines() {
        let tile: Vec<usize> = line.split(",").map(|n| n.parse().unwrap()).collect();
        rt.push((tile[0], tile[1]));
        hs.insert((tile[0], tile[1]));
    }

    let gt = map_green_tiles(&rt);

    Grid(hs, gt)
}

pub fn solve_part_one(data: &str) -> usize {
    let hs = parse_input(data);
    let mut max = 0;
    for a in hs.0.iter() {
        for b in hs.0.iter() {
            
            match area(a, b) {
                Some(v) => {
                    if v > max {
                        max = v;
                    }
                },
                None => {}
            }
        }
    } 

    max
}

pub fn solve_part_two(data: &str) -> usize {
    let mut gt: HashSet<(usize, usize)> = HashSet::new();
    let Grid(rt, gt) = parse_input(data);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &'static str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(SAMPLE_DATA), 50);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(SAMPLE_DATA), 24);
    }
}