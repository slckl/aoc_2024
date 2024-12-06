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

    pub fn at(&self, x: i32, y: i32) -> Option<bool> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(self.obstacles[(x + y * self.width) as usize])
        } else {
            None
        }
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

#[derive(Debug, Clone, Copy)]
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

fn guard_walk(map: &Map, mut guard_pos: (i32, i32), mut dir: Direction) -> Vec<(i32, i32)> {
    let mut path = vec![guard_pos];
    loop {
        let delta = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        };
        guard_pos.0 += delta.0;
        guard_pos.1 += delta.1;

        if let Some(obstacle) = map.at(guard_pos.0, guard_pos.1) {
            if obstacle {
                // Obstacle, can't go there!
                // Change dir, position remains old.
                dir = dir.next();
                guard_pos.0 -= delta.0;
                guard_pos.1 -= delta.1;
            }
        } else {
            // None means we out of bounds baby.
            break;
        }
        path.push(guard_pos);
    }
    path
}

fn p1(i: &str) -> usize {
    let (map, guard_x, guard_y) = Map::parse(i);
    let positions = guard_walk(&map, (guard_x, guard_y), Direction::Up);
    // Filter distinct positions visited by the guard.
    let distinct: HashSet<_> = positions.into_iter().collect();
    distinct.len()
}

#[test]
fn test_walk() {
    let distinct = p1(MAP);
    assert_eq!(distinct, 41);
}

fn main() {
    // Part 1.
    let input = read_to_string("day6/input.txt").unwrap();
    println!("{}", p1(&input));
}
