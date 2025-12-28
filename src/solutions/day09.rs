use core::panic;
use std::{cmp::Ordering, collections::{HashMap, HashSet}, fmt::{Debug, Display}, ops::{Range, RangeInclusive}, usize};


struct Grid {
    tiles: HashSet<(usize, usize)>,
    tile_paths: HashSet<(usize, usize)>
}

struct Point(usize, usize);
struct Edge {
    start: Point,
    end: Point,
}

impl Point {
    fn in_between(&self, edge: &Edge) -> bool {
        let (sx, sy) = (edge.start.0, edge.start.1);
        let (ex, ey) = (edge.end.0, edge.end.1);
        
        // Vertical edge (same x)
        if sx == ex && self.0 == sx {
            return self.1 > sy.min(ey) && self.1 < sy.max(ey)
        }
        
        // Horizontal edge (same y)
        if sy == ey && self.1 == sy {
            return self.1 > sx.min(ex) && self.1 < sx.max(ex)
        }
        
        false
    }
}

impl Grid {
    fn min_max(&self) -> ((usize, usize), (usize, usize)) {
        let mut a: Vec<&(usize, usize)> = self.tile_paths.iter().collect();
        a.sort_by(|a, b| a.1.cmp(&b.1));
        let x = (
            a.first().unwrap().1,
            a.last().unwrap().1
        );

        a.sort_by(|a,b| a.0.cmp(&b.0));

        (
            x,
            (
                a.first().unwrap().0,
                a.last().unwrap().0
            )
        )
    }

    fn map_boundaries(&self) -> HashMap<usize, Vec<RangeInclusive<usize>>> {
        let mut hm = HashMap::new();
        for (x, y) in self.tiles.iter() {
            
        }
        hm
    }

    fn get_all_edges(&self, min_max: (usize, usize), y: usize) -> Vec<RangeInclusive<usize>> {
        let mut edges = vec![];
        let (mut start, mut end) = min_max;
        let mut within_range = false;
        println!("{:?}", self.tiles);
        for x in start..=end {
            println!("{x}, {y} {within_range}");
            if self.tiles.contains(&(x, y)) && self.tiles.contains(&(x + 1, y)) {
                continue;
            }
            
            if self.tiles.contains(&(x, y)) {
                if within_range {
                    edges.push(start..=end);
                } else {
                    start = y;
                }
                within_range = !within_range;
            }
        }
        edges
    }

    fn path(&self, bounds: [(usize, usize); 4]) -> Vec<(usize, usize)> {
        let mut p = vec![];
        let mut i = 0;
        while i < 4 {
            let mut path = if i == 3 {
                create_path(&bounds[3], &bounds[0])
            } else {
                create_path(&bounds[i], &bounds[i + 1])
            };
            i += 1;

            p.append(&mut path);
        }
        p
    }

    fn candidates(&self) -> Vec<([(usize, usize); 4], usize)> {
        let mut points = vec![];
        let tiles: Vec<&(usize, usize)> = self.tile_paths.iter().collect();

        for rx in tiles.iter() {
            for rs in tiles.iter() {
                if rx == rs {
                    continue;
                }
                let size = area(&rx, &rs).unwrap();
                let mut bounds = [**rx, (rx.0, rs.1), **rs, (rs.0, rx.1)];
                println!("bounds before sort: {bounds:?}");
                bounds.sort_by(|a,b| {
                    match a.1.cmp(&b.1) {
                        Ordering::Equal => a.0.cmp(&b.0),
                        other => other
                    }
                });
                println!("bounds after sort {bounds:?}");
                points.push(([bounds[0], bounds[1], bounds[3], bounds[2]], size));
            }
        }

        println!("cobbled together {} candidates", points.len());

        points.sort_by(|a, b| {
            a.1.cmp(&b.1)
        });

        println!("sorted candidates");
        points.reverse();
        println!("reversed candidates");
        points
    }

    fn contains(&self, origin: &(usize, usize), max: &usize) -> usize {
        let mut point = *origin;
        let mut count = 0;
        // println!("checking containment for point: {origin:?}");
        if self.tiles.contains(&origin) {
            return 1
        }

        loop {
            if point.0 > *max {
                break;
            }

            while self.tiles.contains(&point) && self.tiles.contains(&(point.0 + 1, point.1)) {
                point = (point.0 + 1, point.1);
            }

            if self.tiles.contains(&point) {
                count += 1;
            }
            
            point = (point.0 + 1, point.1);
        }
        // println!("point {origin:?} crossed {} tiles", count);

        count
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut max_x_bound = 0;
        let mut max_y_bound = 0;
        let mut min_x_bound = usize::MAX;
        let mut min_y_bound = usize::MAX;
        for (x, y) in self.tiles.iter() {
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
        for y in 0..=(max_y_bound + 2) {
            for x in 0..=(max_x_bound + 2) {
                if self.tiles.contains(&(x, y)) && !self.tile_paths.contains(&(x, y)) {
                    grid.push('X');
                } else if self.tiles.contains(&(x, y)) {
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
    let r = 1..3;

    Some(height * width)
}

fn create_path(origin: &(usize, usize), dest: &(usize, usize)) -> Vec<(usize, usize)> {
    if origin.0 == dest.0 && origin.1 < dest.1 {
        // left to right (y increases)
        ((origin.1 + 1)..=dest.1).map(|n| (origin.0, n)).collect()
    } else if origin.0 == dest.0 && origin.1 > dest.1 {
        // right to left (y decreases)
        (dest.1..origin.1).rev().map(|n| (origin.0, n)).collect()
    } else if origin.1 == dest.1 && origin.0 < dest.0 {
        // top to bottom (x increases)
        ((origin.0 + 1)..=dest.0).map(|n| (n, origin.1)).collect()
    } else {
        // bottom to top (x decreases)
        (dest.0..origin.0).rev().map(|n| (n, origin.1)).collect()
    }
}

fn tile_paths(
    tiles: &Vec<(usize, usize)>,
    tileset: &mut HashSet<(usize, usize)>
) -> HashSet<(usize, usize)> {
    let mut tile_paths = HashSet::new();
    let rt: Vec<&(usize, usize)> = tiles.iter().collect();
    for slice in rt.windows(2) {
        let origin = slice[0];
        let dest = slice[1];
        // same row, but runs left to right
        let path: Vec<(usize, usize)> = create_path(origin, dest);
        
        for p in &path {
            tileset.insert(*p);
        }
        tile_paths.insert(*origin);
    }
    let first = rt[0];
    let last = rt[rt.len() - 1];
    let path = create_path(last, first);
    for p in &path {
        tileset.insert(*p);
    }

    tile_paths
}

fn parse_input(data: &str) -> Grid {
    let mut hs: HashSet<(usize, usize)> = HashSet::new();
    let mut rt: Vec<(usize, usize)> = vec![];
    for line in data.lines() {
        let tile: Vec<usize> = line.split(",").map(|n| n.parse().unwrap()).collect();
        rt.push((tile[0], tile[1]));
        hs.insert((tile[0], tile[1]));
    }

    let gt = tile_paths(&rt, &mut hs);

    Grid { tiles: hs, tile_paths: gt }
}

pub fn solve_part_one(data: &str) -> usize {
    let hs = parse_input(data);
    let mut max = 0;
    for a in hs.tiles.iter() {
        for b in hs.tiles.iter() {
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
    let mut grid = parse_input(data);
    let max = grid
        .tile_paths
        .iter()
        .max_by(|a,b| a.0.cmp(&b.0)).unwrap().0;
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

    #[test]
    fn test_sort() {
        let mut bounds = [(1, 1), (10, 1), (10, 10), (1, 10)];
        let grid = parse_input(SAMPLE_DATA);
        let hm = grid.map_boundaries();
        println!("{grid:?}");

    }
}