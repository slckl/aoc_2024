use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, PartialEq)]
struct Map {
    objects: Vec<i32>,
    width: i32,
    height: i32,
}

impl Map {
    pub fn parse(i: &str) -> Self {
        let mut objects = Vec::with_capacity(i.len());
        let mut height = 0i32;
        let mut width = 0i32;
        let lines = i.lines();
        for line in lines {
            width = line.len() as i32;
            height += 1;
            for ch in line.chars() {
                let obj = match ch {
                    '.' => -1,
                    ch => ch.to_digit(10).unwrap() as i32,
                };
                objects.push(obj);
            }
        }
        Map {
            objects,
            width,
            height,
        }
    }

    fn idx(&self, x: i32, y: i32) -> usize {
        (x + y * self.width) as usize
    }

    pub fn at(&self, x: i32, y: i32) -> Option<i32> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(self.objects[self.idx(x, y)])
        } else {
            None
        }
    }
}

#[cfg(test)]
const TEST_MAP: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

#[test]
fn test_parse() {
    let map = Map::parse(TEST_MAP);
    assert_eq!(map.at(2, 0), Some(0));
    assert_eq!(map.at(7, 7), Some(2));
}

fn eval_trailhead(map: &Map, pos: (i32, i32)) -> HashSet<Vec<(i32, i32)>> {
    let mut candidates = vec![
        ((pos.0 - 1, pos.1), vec![pos], 0),
        ((pos.0, pos.1 - 1), vec![pos], 0),
        ((pos.0 + 1, pos.1), vec![pos], 0),
        ((pos.0, pos.1 + 1), vec![pos], 0),
    ];
    let mut scoring_paths: HashSet<Vec<(i32, i32)>> = HashSet::new();
    while let Some((cpos, mut prev_pos, prev_score)) = candidates.pop() {
        let Some(step) = map.at(cpos.0, cpos.1) else {
            continue;
        };
        let prev_pos_last = prev_pos.last().copied().unwrap();
        // println!(
        //     "step ({}, {}): {} from ({}, {}): {}",
        //     cpos.0, cpos.1, step, prev_pos_last.0, prev_pos_last.1, prev_score
        // );
        if prev_score + 1 == step {
            // println!("step is increment of 1");
            if step == 9 {
                prev_pos.push(cpos);
                scoring_paths.insert(prev_pos);
                continue;
            } else {
                prev_pos.push(cpos);
                let path_so_far = prev_pos;
                // Add new candidates.
                let new_candidates = [
                    ((cpos.0 - 1, cpos.1), path_so_far.clone(), step),
                    ((cpos.0, cpos.1 - 1), path_so_far.clone(), step),
                    ((cpos.0 + 1, cpos.1), path_so_far.clone(), step),
                    ((cpos.0, cpos.1 + 1), path_so_far.clone(), step),
                ];
                for candidate in new_candidates {
                    if candidate.0 != prev_pos_last {
                        // println!("+ candidate: {candidate:?}");
                        candidates.push(candidate);
                    }
                }
            }
        }
    }
    scoring_paths
}

#[test]
fn test_eval() {
    let map = Map::parse(TEST_MAP);
    let paths: HashSet<_> = eval_trailhead(&map, (2, 0))
        .into_iter()
        .map(|path| path.last().cloned().unwrap())
        .collect();
    assert_eq!(paths.len(), 5);
}

fn eval_all_trailheads_p1(map: &Map) -> usize {
    let mut sum = 0;
    for x in 0..map.width {
        for y in 0..map.height {
            if map.at(x, y).unwrap() == 0 {
                let paths: HashSet<_> = eval_trailhead(map, (x, y))
                    .into_iter()
                    .map(|path| path.last().cloned().unwrap())
                    .collect();

                sum += paths.len();
            }
        }
    }
    sum
}

fn eval_all_trailheads_p2(map: &Map) -> usize {
    let mut sum = 0;
    for x in 0..map.width {
        for y in 0..map.height {
            if map.at(x, y).unwrap() == 0 {
                sum += eval_trailhead(map, (x, y)).len();
            }
        }
    }
    sum
}

fn main() {
    let input = read_to_string("day10/input.txt").unwrap();
    let map = Map::parse(&input);
    println!("p1: {}", eval_all_trailheads_p1(&map));
    println!("p2: {}", eval_all_trailheads_p2(&map));
}
