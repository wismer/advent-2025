use core::f64;
use std::{collections::{HashMap, HashSet}, fmt::Debug};

#[derive(Debug, Clone, Copy, Hash, Eq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
    name: char
}
struct Circuit {
    points: HashSet<Point>
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Debug for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buff = String::new();
        for pt in self.points.iter() {
            let s = format!("{:?}\n", &pt);
            buff.push_str(&s);
        }
        write!(f, "\n{}", buff)
    }
}

impl Point {
    fn distance_to(&self, other: &Point) -> usize {
        let x = self.x.abs_diff(other.x);
        let y = self.y.abs_diff(other.y);
        let z = self.z.abs_diff(other.z);
        f64::sqrt(
            ((x * x) + (y * y) + (z * z)) as f64
        ) as usize
    }
}

impl Circuit {
    fn contains(&self, point: &Point) -> bool {
        self.points.contains(point)
    }

    fn size(&self) -> usize {
        self.points.len()
    }

    fn connect(&mut self, other: Point) {
        self.points.insert(other);
    }
}



fn parse_input(data: &str) -> (HashMap<(Point, Point), usize>, usize) {
    let mut points: Vec<Point> = vec![];
    let a: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string().chars().collect();
    let coordinates: Vec<_> = data.lines().enumerate().collect();
    for (i, line) in coordinates.iter() {
        let pts: Vec<usize> = line
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();

        points.push(Point { x: pts[0], y: pts[1], z: pts[2], name: 's' });

        // points.push(Circuit { points: vec![Point { x: pts[0], y: pts[1], z: pts[2] }] });
    }
    let mut hm: HashMap<(Point, Point), usize> = HashMap::new();
    let mut distances_by_points: Vec<(usize, Point, Point)> = vec![];
    let copy = points.clone();

    for i in 0..points.len() {
        let pt = copy[i];

        for p in points.iter() {
            if pt.x == p.x && pt.y == p.y && pt.z == p.z {
                continue;
            }
            let distance = pt.distance_to(&p);
            let pt_a = Point { x: pt.x, y: pt.y, z: pt.z, name: pt.name };
            let pt_b = Point { x: p.x, y: p.y, z: p.z, name: p.name };
            let already_here = hm.contains_key(&(pt_b, pt_a));

            // println!("{pt_a:?} -> {pt_b:?} ({distance})");
            if already_here {
                continue;
            }

            match hm.get_mut(&(pt_a, pt_b)) {
                Some(v) => {
                    if distance < *v {
                        // println!("replacing {:?} with {:?}", v, (distance, pt_b));
                        *v = distance;
                    } else {
                    }
                },
                None => {
                    hm.insert((pt_a, pt_b), distance);
                }
            }

            distances_by_points.push((distance, Point { x: pt.x, y: pt.y, z: pt.z, name: pt.name }, Point { x: p.x, y: p.y, z: p.z, name: p.name }));            
        }
    }
    for key in hm.keys() {
        // println!("key: {key:?} value: {:?}", hm.get(key).unwrap());
    }
    // distances_by_points.sort_by(|a,b| a.0.cmp(&b.0));
    // println!("{:?}", hm);

    (hm, coordinates.len())
}

pub fn solve_part_one(data: &str) -> usize {
    let (mut pts, size) = parse_input(data);
    let mut total = 1;
    let mut min = f64::MAX;
    let mut circuits: Vec<Circuit> = vec![];
    // println!("{:?}", &pts[..10]);
    // start with shortest distance first
    let mut ks: Vec<_> = pts.keys().collect();
    let mut keys: Vec<_> = ks.iter().map(|k| {
        let v = pts.get(k).unwrap();
        (v, k)
    }).collect();
    keys.sort_by(|a,b| a.0.cmp(&b.0));
    let mut pairs_made = 0;
    for (d, k) in keys {

        if pairs_made > (size / 2) {
            println!("{:?}, ", k);
            break;
        }
        
        
        
        match pts.get(k) {
            Some(v) => {
                println!("attempting getting {k:?} -> {v:?} -> ");
                let circuit_contains_point = circuits.iter_mut().find(|c| c.contains(&k.0) || c.contains(&k.1));
                match circuit_contains_point {
                    Some(c) => {
                        match (c.contains(&k.0), c.contains(&k.1)) {
                            (true, false) => {
                                pairs_made += 1;
                                c.connect(k.1);
                            },
                            (false, true) => {
                                pairs_made += 1;
                                c.connect(k.0);
                            },
                            _ => {}
                        }
                    },
                    None => {
                        pairs_made += 1;
                        println!("connections made: {pairs_made}");
                        let mut points = HashSet::new();
                        points.insert(k.0);
                        points.insert(k.1);
                        let circut = Circuit { points };
                        circuits.push(circut);
                    }
                }
            },
            None => {}
        }
    }

    circuits.sort_by(|a,b| a.size().cmp(&b.size()));
    circuits.reverse();
    for c in circuits.iter().take(3) {
        println!("{}", c.size());
        if c.size() > 0 {
            total *= c.size();
        }
    }
    // values.sort_by(|a,b| a.0.cmp(&b.0));
    // for v in values {
    //     match pts.get(v.1) {
    //         Some()
    //     }
    // }
    // for (pt_a, (d, pt_b)) in pts {
    //     for circuit in circuits.iter_mut() {
    //         if circuit.contains(&pt_b) {
    //             circuit.connect(pt_a);
    //         }
    //         continue;
    //     }
    // }
    
    // println!("circuits {:?}", circuits);
    total

}

pub fn solve_part_two(data: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &'static str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(SAMPLE_DATA), 40);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(SAMPLE_DATA), 0);
    }
}