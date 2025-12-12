use core::f64;
use std::{collections::{HashMap, HashSet}, fmt::Debug};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
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

impl Debug for JunctionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buff = String::new();
        for r in self.boxes.iter() {
            for n in r.iter().map(|f| (f.x, f.y, f.z)) {
                let s = format!("{n:?},");
                buff.push_str(&s);
            }
            buff.push('\n');
        }

        write!(f, "{}", buff)
    }
}

impl Point {
    fn distance_to(&self, other: &Point) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        f64::sqrt(
            ((x * x) + (y * y) + (z * z)) as f64
        )
    }
}

// impl Circuit {
//     fn contains(&self, point: &Point) -> bool {
//         self.points.contains(point)
//     }

//     fn size(&self) -> usize {
//         self.points.len()
//     }

//     fn connect(&mut self, other: Point) {
//         self.points.insert(other);
//     }
// }

struct JunctionSet {
    boxes: Vec<Vec<Point>>
}

impl JunctionSet {
    fn contains(&self, pt_a: &Point, pt_b: &Point) -> (Option<usize>, Option<usize>) {
        for (i, b) in self.boxes.iter().enumerate() {
            if b.contains(pt_a) && b.contains(pt_b) {
                return (Some(i), Some(i))
            } else if b.contains(pt_a) {
                return (Some(i), None)
            } else if b.contains(pt_b) {
                return (None, Some(i))
            }
        }

        (None, None)
    }

    fn push(&mut self, pts: Vec<Point>) {
        self.boxes.push(pts);
    }
}


fn parse_input(data: &str) -> (HashMap<Point, (Point, usize)>, usize) {
    let mut points: Vec<Point> = vec![];
    let alpha: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let coordinates: Vec<_> = data.lines().collect();
    for (i, line) in coordinates.iter().enumerate() {
        let pts: Vec<f64> = line
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();

        points.push(Point { x: pts[0], y: pts[1], z: pts[2], name: alpha[i] });
    }

    let mut hm: HashMap<Point, (Point, usize)> = HashMap::new();
    let copy = points.clone();
    let mut x: Vec<Vec<(f64, Point, Point)>> = vec![];
    for (copy_idx, pt_a) in copy.iter().enumerate() {
        let mut ns: Vec<(f64, Point, Point)> = vec![];
        for (idx, pt_b) in points.iter().enumerate() {
            if copy_idx == idx {
                continue;
            }
            let distance = pt_a.distance_to(pt_b);
            let x_distance = pt_b.distance_to(pt_a);
            let dup = x.iter().any(|i| {
                i[0] == (distance, *pt_b, *pt_a)
            });
            if dup {
                println!("dup found A: {:?} B: {:?}", pt_a, pt_b);
                continue;
            }

            ns.push((distance, *pt_a, *pt_b));
            // match hm.get_mut(pt_a) {
            //     Some(p) => {
            //         if p.1 > distance {
            //             *p = (*pt_b, distance);
            //         }
            //     },
            //     None => {
            //         hm.insert(*pt_a, (*pt_b, distance));
            //     }
            // }
        }
        ns.sort_by(|a,b| a.0.total_cmp(&b.0));
        x.push(ns);
    }
    x.sort_by(|a,b| {
        a[0].0.total_cmp(&b[0].0)
    });
    for s in x.iter() {
        println!("{:?}", s[0]);
    }
    let mut list: Vec<(&Point, &(Point, usize))> = hm.iter().map(|(k, v)| {
        (k, v)
    }).collect();

    list.sort_by(|a,b| {
        a.1.1.cmp(&b.1.1)
    });
    // println!("size: {}", list.len());
    for l in list.iter() {
        println!("{l:?}");
    }

    
    // if k1 exists as a value in v2, and k2 exists as a value in v1
    // then that is a joined pair
    let mut ptr: Point = Point { x: 1.0, y: 1.0, z: 1.0, name: 'a' };
    let mut juncs = JunctionSet { boxes: vec![] };
    let mut limit = 20;
    for (k, v) in list.iter() {
        if v.0 == ptr {
            println!("{k:?}");
            continue;
        }
        // two rules:
        // if a's closest neighbor is b, and b's closest neighbor is a, then that is a pair
        // if a's closest neighbor belongs in a pair, then it connects to that pair
        // match hm.get(&v.0) {
        //     Some(p) => {
        //         if p.0 == **k {
        //             println!("SAME: {v:?} -> {k:?}");
        //         } else {
        //             println!("{:?} {v:?}", k);
        //         }
        //     },
        //     None => {
        //         println!("{:?} {v:?}", k);
        //     }
        // }

        match (juncs.contains(k, &v.0)) {
            (None, Some(i)) => {
                match juncs.boxes.get_mut(i) {
                    Some(r) => {
                        limit -= 1;
                        r.push(**k);
                    },
                    None => {}
                }
            },
            (Some(i), None) => {
                match juncs.boxes.get_mut(i) {
                    Some(r) => {
                        limit -= 1;
                        r.push(v.0);
                    },
                    None => {}
                }
            },
            (None, None) => {
                limit -= 1;
                let c: Vec<Point> = vec![v.0, **k];
                juncs.push(c);
            },
            _ => {}
        }


        ptr = **k;
    }

    println!("{juncs:?} {limit}");
    (hm, coordinates.len())
}

pub fn solve_part_one(data: &str) -> usize {
    let (mut pts, size) = parse_input(data);
    let mut total = 1;
    let mut min = f64::MAX;

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