use core::f64;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64
}
struct Circuit {
    points: Vec<Point>
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
    fn distance_to(&self, other: &Point) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        f64::sqrt(
            (x * x) + (y * y) + (z * z)
        )
    }
}

impl Circuit {
    fn shortest(&self, point: &Point) -> f64 {
        let mut min = f64::MAX;
        for pt in self.points.iter() {
            let distance = pt.distance_to(point);
            if distance < min {
                min = distance;
            }
        }

        min
    }

    fn connect(&mut self, other: Point) {
        self.points.push(other);
    }
}



fn parse_input(data: &str) -> Vec<Point> {
    let mut circuits: Vec<Point> = vec![];

    for line in data.lines() {
        let pts: Vec<f64> = line
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();

        circuits.push(Point { x: pts[0], y: pts[1], z: pts[2] });

        // circuits.push(Circuit { points: vec![Point { x: pts[0], y: pts[1], z: pts[2] }] });
    }

    circuits
}

pub fn solve_part_one(data: &str) -> usize {
    let mut pts = parse_input(data);
    let mut idx = 0;
    let mut min = f64::MAX;
    let mut circuits: Vec<Circuit> = vec![];

    while let Some(pt) = pts.pop() {
        let mut circuit = Circuit { points: vec![] };
        // check first with remaining pts for closest distance

        let closest_pt = pts
            .iter()
            .map(|p| p.distance_to(&pt))
            .enumerate()
            .min_by_key(|p| p.1 as usize);
        let closest_circuit = circuits
            .iter()
            .map(|c| c.shortest(&pt))
            .enumerate()
            .min_by_key(|c| c.1 as usize);

        match (closest_circuit, closest_pt) {
            (Some(c), Some(p)) => {
                if c.1 < p.1 {
                    // an existing circuit has a closer pt than the remaining ungrouped pts
                    let mut closest_circuit = circuits.get_mut(c.0).unwrap();
                    closest_circuit.connect(pt);
                } else {
                    // a unpopped pt is closer to the pt
                    let removed_point = pts.remove(p.0);
                    circuit.connect(removed_point);
                    circuit.connect(pt);
                }
            },
            (Some(c), None) => {
                let mut closest_circuit = circuits.get_mut(c.0).unwrap();
                closest_circuit.connect(pt);
            },
            (None, Some(p)) => {
                let removed_point = pts.remove(p.0);
                circuit.connect(removed_point);
                circuit.connect(pt);
            }
        }


        match closest_pt {
            Some(p) => {
                let removed_pt = pts.remove(p.0);
                circuit.connect(pt);
                circuit.connect(removed_pt);
                circuits.push(circuit);
            },
            None => break
        }

    }
    println!("circuits: {circuits:?}");
    0

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