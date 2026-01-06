use core::panic;
use std::{cmp::Ordering, collections::{HashMap, HashSet}, fmt::{Debug, Display}, ops::RangeInclusive, usize};


struct Grid {
    tiles: HashSet<(usize, usize)>,
    tile_paths: HashSet<(usize, usize)>
}


impl IntoIterator for Grid {
    type Item = [(usize, usize); 4];
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let tiles: Vec<&(usize, usize)> = self.tiles.iter().collect();
        let mut sets: Vec<[(usize, usize); 4]> = vec![];
        for rx in tiles.iter() {
            for rs in tiles.iter() {
                if rx == rs {
                    continue;
                }
                
                let set: [(usize, usize); 4] = [**rx, (rx.0, rs.1), **rs, (rs.0, rx.1)];
                sets.push(set);
            }
        }

        sets.into_iter()
    }
}

enum Direction {
    Right,
    Down
}

impl Grid {
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
                // println!("bounds before sort: {bounds:?}");
                bounds.sort_by(|a,b| {
                    match a.1.cmp(&b.1) {
                        Ordering::Equal => a.0.cmp(&b.0),
                        other => other
                    }
                });

                points.push(([bounds[0], bounds[1], bounds[3], bounds[2]], size));
                // println!("cobbled together {:?} candidates", [bounds[0], bounds[1], bounds[3], bounds[2]]);
            }
        }


        points.sort_by(|a, b| {
            a.1.cmp(&b.1)
        });

        println!("sorted candidates");
        points.reverse();
        println!("reversed candidates");
        points
    }

    fn find_contained_point(&self, max: usize) -> Option<(usize, usize)> {
        for k in self.tile_paths.iter() {
            let mut pt = (k.0 + 1, k.1);

            // loop {
            //     if self.tiles.contains(&pt) {
            //         pt = (pt.0 + 1, pt.1);
            //     } else if self.contains(&pt, &max) {
            //         println!("{pt:?} found");
            //         return Some(pt);
            //     } else {
            //         break
            //     }
            // }
        }

        return None
    }

    fn flood_fill(&mut self, origin: &(usize, usize)) {
        // let pt = *origin;
        let mut stack = vec![*origin];
        while let Some(pt) = stack.pop() {
            if self.tiles.contains(&pt) {
                continue;
            }

            // println!("size: {}", self.tiles.len());
            self.tiles.insert(pt);

            let adj_pts = [
                (pt.0 - 1, pt.1),
                (pt.0 + 1, pt.1),
                (pt.0, pt.1 - 1),
                (pt.0, pt.1 + 1)
            ];

            for p in adj_pts {
                if !self.tiles.contains(&p) {
                    stack.push(p);
                }
            }
        }
    }

    fn contains(&self, origin: &(usize, usize), limit: &usize, modifier: (isize, isize)) -> bool {
        let mut point = *origin;
        let mut count = 0;

        if self.tiles.contains(&point) {
            point = ((point.0 as isize + modifier.0) as usize, (point.1 as isize + modifier.1) as usize);
            // println!("from: {origin:?} to: {point:?}");
            // return true
        }

        let show_me = true; // origin == &(2, 3);

        loop {

            if show_me {
                println!("help : {point:?} count : {count} limit: {limit}");
            }

            match modifier {
                (0, -1) => if point.1 < *limit { break },
                (0, 1) => if point.1 > *limit { break },
                (-1, 0) => if point.0 < *limit { break },
                _ => if point.0 > *limit { break }
            }

            let update_pt = match modifier {
                (0, -1) => |p: &(usize, usize)| { (p.0, p.1 - 1) },
                (0, 1) => |p: &(usize, usize)| { (p.0, p.1 + 1) },
                (1, 0) => |p: &(usize, usize)| { (p.0 + 1, p.1) },
                _ => |p: &(usize, usize)| { (p.0 - 1, p.1) }
            };

            while self.tiles.contains(&point) && self.tiles.contains(&update_pt(&point)) {
                point = update_pt(&point);
            }

            if self.tiles.contains(&point) {
                count += 1;
            }
            
            point = update_pt(&point);
        }
        // println!("point {origin:?} crossed {} tiles", count);

        count % 2 != 0
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
    fn test_contains() {
        let mut grid = parse_input(SAMPLE_DATA);
        // for c in [(9, 5), (2, 5), (2, 3), (9, 3)] {
        //     grid.tiles.insert(c);
        // }
        println!("{:?}", grid.tiles);

        let pt = grid.find_contained_point(12);
        match pt {
            Some(p) => {
                grid.flood_fill(&p);
            },
            None => {}
        }
        // println!("{pt:?}");
        // for c in [(9, 5), (2, 5), (2, 3), (9, 3)].iter() {
        //     if !grid.contains(c, &12) {
        //         println!("{c:?}");
        //     }
        // }

        println!("{grid:?}");
        // grid.tiles.insert((5, 7));
        // for c in grid.candidates() {
        //     let path = grid.path(c.0);
        //     if path.iter().all(|c| grid.contains(origin, max))
        // }
        // assert!(grid.contains(&(2, 3)));
    }
}