use core::f64;
use std::fmt::Debug;


#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Point {
    fn distance_to(&self, other: &Point) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        f64::sqrt(
            x.powf(2.0) +
            y.powf(2.0) +
            z.powf(2.0)
        )
    }
}


fn find_circuit(parents: &mut [usize], id: usize) -> usize {
    // println!("looking for {} - {}", id, parents[id]);
    if parents[id] != id {
        parents[id] = find_circuit(parents, parents[id]);
    }

    parents[id]
}

fn union(parents: &mut [usize], a: usize, b: usize) {
    let a_id = find_circuit(parents, a);
    let b_id = find_circuit(parents, b);

    if a_id != b_id {
        parents[b_id] = a_id;
    }
}

fn parse_input(data: &str) -> Vec<(usize, usize, f64, Point, Point)> {
    let mut points: Vec<Point> = vec![];
    let coordinates: Vec<_> = data.lines().collect();
    for (_, line) in coordinates.iter().enumerate() {
        let pts: Vec<f64> = line
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();
        points.push(Point { x: pts[0], y: pts[1], z: pts[2] });
    }
    let mut sorted: Vec<(usize, usize, f64, Point, Point)> = vec![];
    for idx in 0..points.len() {
        let pt_a = &points[idx];
        for (i, pt_b) in points.iter().enumerate() {
            if i == idx {
                continue
            }

            
            let distance = pt_a.distance_to(pt_b);
            if !sorted.contains(&(i, idx, distance, *pt_b, *pt_a)) {
                sorted.push((idx, i, distance, *pt_a, *pt_b));
            }
        }
    }
    sorted.sort_by(|a,b| a.2.total_cmp(&b.2));
    sorted
}

fn get_root_parent(parents: &[usize], mut idx: usize) -> usize {
    let mut v: usize = 0;
    while let Some(n) = parents.get(idx) {
        if parents[*n] != *n {
            idx = *n;
        } else {
            v = *n;
            break;
        }
    }
    v
}

pub fn solve_part_one(data: &str) -> usize {
    let sorted = parse_input(data);
    let mut parents: Vec<usize> = (0..sorted.len()).collect();
    let mut sizes: Vec<usize> = vec![0; sorted.len()];
    println!("parsed");
    for (_, (pt_a, pt_b, _, a, b)) in sorted.iter().take(10).enumerate() {
        union(&mut parents[..], *pt_a, *pt_b);
    }
    for idx in 0..sorted.len() {
        let parent = find_circuit(&mut parents[..], idx);
        sizes[parent] += 1;
    }

    sizes.sort();
    sizes.reverse();
    sizes.iter().take(3).product()
}

pub fn solve_part_two(data: &str) -> usize {
    let line_count = data.lines().count();
    let sorted = parse_input(data);
    let mut parents: Vec<usize> = (0..sorted.len()).collect();
    for (idx, (pt_a, pt_b, _, a, b)) in sorted.iter().enumerate() {
        union(&mut parents[..], *pt_a, *pt_b);
        let terminating_num = get_root_parent(&parents, 0);
        let all_same = &parents[0..line_count].iter().map(|n| {
            get_root_parent(&parents, *n)
        }).all(|z| z == terminating_num);
        if *all_same {
            return (a.x * b.x) as usize
        }
    }

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
        assert_eq!(solve_part_two(SAMPLE_DATA), 25272);
    }
}