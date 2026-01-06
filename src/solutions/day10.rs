use std::{collections::{HashSet, VecDeque}, fmt::Debug};

fn parse_diagram(data: &str) -> (u32, usize) {
    let mut diagram = 0;
    let size = data.len();
    for (i, c) in data.chars().enumerate() {
        if c == '#' {
            diagram |= 1 << (size - i - 1);
        }
    }

    (diagram, size)
}

fn parse_input(data: &str) -> Vec<Machine> {
    let mut machines = vec![];

    for line in data.lines() {
        // split whitespace
        let mut chunks = line.split_whitespace();
        let mut diagram: (u32, usize) = (0, 0);
        let mut buttons: Vec<Vec<u32>> = vec![];
        let mut joltage: Vec<u32> = vec![];
        while let Some(chunk) = chunks.next() {
            if chunk.starts_with("[") {
                diagram = parse_diagram(&chunk[1..(chunk.len() - 1)]);
            } else if chunk.starts_with('(') {
                let btn: Vec<u32> = chunk[1..(chunk.len() - 1)].split(',').map(|i| i.parse().unwrap()).collect();
                buttons.push(btn);
            } else {
                joltage = chunk[1..(chunk.len() - 1)].split(',').map(|i| i.parse().unwrap()).collect();
            }
        }
        machines.push(Machine {
            light_diagram: diagram.0,
            size: diagram.1,
            buttons,
            joltage_requirements: joltage,
            state: 0,
        });
    }

    machines
}

struct Machine {
    light_diagram: u32,
    buttons: Vec<Vec<u32>>,
    joltage_requirements: Vec<u32>,
    state: u32,
    size: usize,
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();

        for i in 0..self.size {
            if self.state & (1 << i) > 0 {
                buf.push('#');
            } else {
                buf.push('.');
            }
        }

        write!(f, "[{}]", buf)
    }
}

impl Machine {
    fn measure_joltage(&self) -> Option<u32> {
        let mut grid: Vec<Vec<u32>> = vec![];
        let mut sum = 0;
        for i in 0..self.size as u32 {
            let mut row: Vec<u32> = vec![];
            for btn in self.buttons.iter() {
                if btn.contains(&i) {
                    row.push(1);
                    sum += 1;
                } else {
                    row.push(0);
                }
            }

            grid.push(row);
        }

        println!("{:?}", grid);

        Some(sum as u32)
    }


    fn measure_distance(&self) -> Option<u32> {
        let mut queue = VecDeque::new();
        let mut seen: HashSet<u32> = HashSet::new();
        queue.push_back((0, 0u32));
        seen.insert(0);
        
        while let Some((state, steps)) = queue.pop_front() {
            // Check if current state matches target
            if state == self.light_diagram {
                return Some(steps);
            }
            
            for btn in self.buttons.iter() {
                let mut next_state = state;
                for &light_idx in btn {
                    next_state ^= 1 << self.size as u32 - light_idx - 1;
                }

                if !seen.contains(&next_state) {
                    seen.insert(next_state);
                    queue.push_back((next_state, steps + 1));
                }
            }
        }
        None
    }
}

pub fn solve_part_one(data: &str) -> usize {
    let machines = parse_input(data);
    let mut sum = 0;
    // match machines[0].measure_distance() {
    //     Some(n) => sum += n,
    //     None => {}
    // }
    for machine in machines {
        match machine.measure_distance() {
            Some(n) => {
                println!("Found distance: {} for {:?}", n, machine);
                sum += n;
            },
            None => {}
        }

    }

    sum as usize
}

pub fn solve_part_two(data: &str) -> usize {
    let machines = parse_input(data);
    let mut sum = 0;
    // match machines[0].measure_distance() {
    //     Some(n) => sum += n,
    //     None => {}
    // }
    for machine in machines {
        match machine.measure_joltage() {
            Some(n) => {
                println!("Found distance: {} for {:?}", n, machine);
                sum += n;
            },
            None => {}
        }

    }

    sum as usize
}

// idea:
// - may involve permutations
// - but keep track of button presses by index. If even: off, odd: on
// - focus first on the buttons that share the same indexes as the lights that should be ON
// - Buttons can be pressed more than once
// - recursion might be an answer here
// actually this looks like bitwise


#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &'static str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(SAMPLE_DATA), 7);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(SAMPLE_DATA), 33);
    }

    // #[test]
    // fn test_toggle() {
    //     let mut machines = parse_input(SAMPLE_DATA);
    //     let machine = &mut machines[0];
    //     let expected = vec![false, false, false, true];
    //     machine.press_btn(0);
    //     assert_eq!(expected, machine.state);
    // }

    #[test]
    fn test_parse_diagram() {
        let (diagram, size) = parse_diagram(".##..#");
        assert_eq!(0b011001u32, diagram);
        assert_eq!(6usize, size);
    }
}