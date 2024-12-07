use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, PartialEq)]
struct Map {
    obstacles: Vec<bool>,
    width: i32,
    height: i32,
}

impl Map {
    pub fn parse(txt: &str) -> (Self, i32, i32) {
        let mut obstacles = Vec::with_capacity(txt.len());
        let mut height = 0i32;
        let mut width = 0i32;
        let lines = txt.lines();
        let mut guard_pos = (0i32, 0i32);
        for (y, line) in lines.enumerate() {
            width = line.len() as i32;
            height += 1;
            for (x, ch) in line.chars().enumerate() {
                obstacles.push(ch == '#');
                if ch == '^' {
                    guard_pos = (x as i32, y as i32);
                }
            }
        }
        (
            Map {
                obstacles,
                width,
                height,
            },
            guard_pos.0,
            guard_pos.1,
        )
    }

    fn idx(&self, x: i32, y: i32) -> usize {
        (x + y * self.width) as usize
    }

    pub fn at(&self, x: i32, y: i32) -> Option<bool> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(self.obstacles[self.idx(x, y)])
        } else {
            None
        }
    }

    pub fn flip_obstacle(&mut self, x: i32, y: i32) -> bool {
        let idx = self.idx(x, y);
        self.obstacles[idx] = !self.obstacles[idx];
        self.obstacles[idx]
    }
}

#[cfg(test)]
const MAP: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

#[test]
fn test_map_parse() {
    let (map, guard_x, guard_y) = Map::parse(MAP);
    assert_eq!(map.at(0, 0), Some(false));
    assert_eq!(map.at(9, 1), Some(true));
    assert_eq!(guard_x, 4);
    assert_eq!(guard_y, 6);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn next(&self) -> Self {
        match &self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GuardPos {
    x: i32,
    y: i32,
    dir: Direction,
}

impl GuardPos {
    fn new(x: i32, y: i32, dir: Direction) -> Self {
        Self {
            x, y, dir
        }
    }
}

fn guard_walk(map: &Map, mut guard_pos: GuardPos) -> (Vec<GuardPos>, bool) {
    let mut path = vec![guard_pos];
    let mut loopy = false;
    let mut obstacle_approaches = HashSet::new();
    'walk: loop {
        let delta = match guard_pos.dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        };
        guard_pos.x += delta.0;
        guard_pos.y += delta.1;

        if let Some(obstacle) = map.at(guard_pos.x, guard_pos.y) {
            if obstacle {
                // Check for loopiness.
                if obstacle_approaches.contains(&guard_pos) {
                    loopy = true;
                    break 'walk;
                } else {
                    obstacle_approaches.insert(guard_pos);
                }
                // Obstacle, can't go there!
                // Change dir, position remains old.
                guard_pos.dir = guard_pos.dir.next();
                guard_pos.x -= delta.0;
                guard_pos.y -= delta.1;
            }
        } else {
            // None means we out of bounds baby.
            break;
        }
        path.push(guard_pos);
        
    }
    (path, loopy)
}

fn p1(i: &str) -> usize {
    let (map, guard_x, guard_y) = Map::parse(i);
    let (positions, _loopy) = guard_walk(&map, GuardPos::new(guard_x, guard_y, Direction::Up));
    // Filter distinct positions visited by the guard.
    let distinct: HashSet<_> = positions.into_iter().map(|gpos| (gpos.x, gpos.y)).collect();
    distinct.len()
}

#[test]
fn test_walk() {
    let distinct = p1(MAP);
    assert_eq!(distinct, 41);
}

fn p2(i: &str) -> Vec<(i32, i32)> {
    let (mut map, guard_x, guard_y) = Map::parse(i);
    let mut obstacle_positions = Vec::new();
    let (positions, _loopy) = guard_walk(&map, GuardPos::new(guard_x, guard_y, Direction::Up));
    // Analyze every position of the guard's path, whether it could use an obstacle to cause the guard go in a loop.
    for candidate in &positions[1..] {
        // Put an obstacle at the position, play guard walk from here, with a check for loop.
        map.flip_obstacle(candidate.x, candidate.y);
        
        // Idx is idx -1 of positions[0..], so we can use it here.
        // Rerun path from the start to see if we loop.
        let (_new_positions, loopy) = guard_walk(&map, positions[0]);
        if loopy {
            obstacle_positions.push((candidate.x, candidate.y));
        }
        // Unflip so as not to affect other searches.
        map.flip_obstacle(candidate.x, candidate.y);
        // println!("Position ({}, {}) is {}", candidate.x, candidate.y, if loopy { "loopy" } else { "not loopy"} );
    }

    obstacle_positions
}

#[test]
fn test_p2() {
    let obstacle_positions = p2(MAP);
    let distinct_obs_pos: HashSet<_> = obstacle_positions.into_iter().collect();
    assert_eq!(distinct_obs_pos.len(), 6);
}

fn main() {
    // Part 1.
    let input = read_to_string("day6/input.txt").unwrap();
    println!("p1: {}", p1(&input));
    // Part 2.
    let obstacle_positions = p2(&input);
    let distinct_obs_pos: HashSet<_> = obstacle_positions.into_iter().collect();
    println!("p2: {}", distinct_obs_pos.len());
}
