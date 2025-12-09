use std::fmt;

fn parse_input(data: &str) -> Tree {
    let tree_parts: Vec<Vec<TreePart>> = data.lines().map(|line| {
        line.chars().map(|c| {
            match c {
                '^' => TreePart::Tachyon,
                'S' => TreePart::Start,
                _ => TreePart::None
            }
        }).collect()
    }).collect();
    Tree {
        size: tree_parts.len(),
        tree: tree_parts,
        beams: vec![]
    }
}

impl Tree {

    fn get_next_part(&self, coord: (usize, usize)) -> Option<&TreePart> {
        match self.tree.get(coord.0) {
            Some(row) => {
                row.get(coord.1)
            },
            None => None
        }
    }

    fn move_beams(&mut self) -> usize {
        let mut split_count = 0;
        if self.beams.len() == 0 {
            let next = (1, (self.size / 2) - 1);
            self.tree[1][(self.size / 2) - 1] = TreePart::Beam;
            self.beams.push(next);
        } else {
            let coords: Vec<(usize, usize)> = self.beams.drain(..).collect();
            let mut new_beams: Vec<(usize, usize)> = vec![];
            for coord in coords {
                // println!("coord; {:?}", coord);
                match self.get_next_part((coord.0 + 1, coord.1)) {
                    Some(part) => {
                        match part {
                            TreePart::None => {
                                new_beams.push((coord.0 + 1, coord.1));
                            },
                            TreePart::Tachyon => {
                                let left = (coord.0 + 1, coord.1 - 1);
                                let right = (coord.0 + 1, coord.1 + 1);
                                if !new_beams.contains(&left) || !new_beams.contains(&right) {
                                    split_count += 1;

                                    if !new_beams.contains(&left) {
                                        new_beams.push(left);
                                    }

                                    if !new_beams.contains(&right) {
                                        new_beams.push(right);
                                    }
                                }

                            },
                            TreePart::Beam => {
                                println!("already a beam here at {:?}", (coord.0 + 1, coord.1));
                            },
                            _ => {}
                        }
                    },
                    None => {}
                }
            }
            self.beams.append(&mut new_beams);
            for beam in self.beams.iter() {
                // println!("beam: {:?}", beam);
                self.tree[beam.0][beam.1] = TreePart::Beam;
            }
        }

        println!("split: {split_count:?}");
        split_count
    }
}

pub fn solve_part_one(data: &str) -> usize {
    let mut total = 0;
    let mut tree = parse_input(data);
    for _ in 0..tree.size {
        total += tree.move_beams();
        println!("{tree:?}");
    }


    total
}

pub fn solve_part_two(data: &str) -> usize {
    // factor in the overlaps
    let mut total = 0;
    let mut tree = parse_input(data);
    for _ in 0..tree.size {
        total += tree.move_beams();
        println!("{tree:?}");
    }


    total
}

#[derive(Debug)]
enum TreePart {
    Beam,
    Start,
    Tachyon,
    None
}
struct Tree {
    tree: Vec<Vec<TreePart>>,
    pub size: usize,
    pub beams: Vec<(usize, usize)>
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tree.iter().map(|row| {
            row.iter().map(|part| {
                match part {
                    TreePart::Start => 'S',
                    TreePart::Beam => '|',
                    TreePart::Tachyon => '^',
                    _ => '.'
                }
            }).collect::<String>()
        }).collect::<Vec<String>>().join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &'static str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_solve_part_one() {
        assert_eq!(solve_part_one(SAMPLE_DATA), 21);
    }

    #[test]
    fn test_solve_part_two() {
        assert_eq!(solve_part_two(SAMPLE_DATA), 40);
    }
}