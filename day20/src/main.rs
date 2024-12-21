use std::{collections::{HashMap, HashSet}, fs::read_to_string};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Track {
    Start,
    End,
    Track,
    Wall,
}

#[derive(Debug, PartialEq)]
struct Map {
    objects: Vec<Track>,
    width: i32,
    height: i32,
    start: (i32, i32),
    end: (i32, i32),
}

impl Map {
    pub fn parse(i: &str) -> Self {
        let mut objects = Vec::with_capacity(i.len());
        let mut height = 0i32;
        let mut width = 0i32;
        let lines = i.lines();
        let mut start = None;
        let mut end = None;
        for line in lines {
            width = line.len() as i32;
            height += 1;
            for (x_idx, ch) in line.chars().enumerate() {
                let obj = match ch {
                    '.' => Track::Track,
                    'S' => {
                        start = Some((x_idx as i32, height - 1));
                        Track::Start
                    }
                    'E' => {
                        end = Some((x_idx as i32, height - 1));
                        Track::End
                    }
                    '#' => Track::Wall,
                    ch => panic!("Unexpected char: {ch}"),
                };
                objects.push(obj);
            }
        }
        Map {
            objects,
            width,
            height,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn idx(&self, x: i32, y: i32) -> usize {
        (x + y * self.width) as usize
    }

    pub fn at(&self, x: i32, y: i32) -> Option<Track> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(self.objects[self.idx(x, y)])
        } else {
            None
        }
    }
}

#[cfg(test)]
const TEST_MAP: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

#[test]
fn test_parse() {
    let map = Map::parse(TEST_MAP);
    assert_eq!(map.at(0, 0), Some(Track::Wall));
    assert_eq!(map.at(1, 3), Some(Track::Start));
}

fn add_next(x: i32, y: i32, map: &Map, out: &mut Vec<(i32, i32)>, shortcut: Option<(i32, i32)>) {
    if let Some(track) = map.at(x, y) {
        match track {
            Track::Track => out.push((x, y)),
            Track::End => out.push((x, y)),
            Track::Wall => {
                if shortcut.is_some() && (x, y) == shortcut.unwrap() {
                    out.push((x, y));
                }
            }
            _ => (),
        }
    }
}

fn path(map: &Map, shortcut: Option<(i32, i32)>) -> Vec<(i32, i32)> {
    // Start at map.start.
    // Init with all the positions around it.
    // let path = pathfinding::directed::dfs::dfs(
        let path = pathfinding::directed::bfs::bfs(
        &map.start,
        |&(x, y)| {
            // println!("next for {x}, {y}");
            // Check up, down, left, right
            let mut next = Vec::with_capacity(4);
            add_next(x - 1, y, map, &mut next, shortcut);
            add_next(x + 1, y, map, &mut next, shortcut);
            add_next(x, y - 1, map, &mut next, shortcut);
            add_next(x, y + 1, map, &mut next, shortcut);
            // println!("next: {next:?}");
            next
        },
        |pos| *pos == map.end,
    )
    .unwrap();

    path
}

#[test]
fn test_path() {
    let map = Map::parse(TEST_MAP);
    // - 1, cause we include the end in the list.
    assert_eq!(path(&map, None).len() - 1, 84);
    assert_eq!(path(&map, Some((8, 1))).len() - 1, 72);
}

fn add_wall(x: i32, y: i32, map: &Map, wall_set: &mut HashSet<(i32, i32)>) {
    if let Some(track) = map.at(x, y) {
        if track == Track::Wall {
            {
                let _x = wall_set.insert((x, y));
            }
        }
    }
}

fn count_cheats(map: &Map, threshold: usize) -> usize {
    // Obtain canonical path.
    let canon = path(map, None);
    let canon_len = canon.len() - 1;
    println!("canon_len: {canon_len}");
    // List all walls along the path.
    let mut walls = HashSet::new();
    for (x, y) in canon {
        add_wall(x - 1, y, map, &mut walls);
        add_wall(x + 1, y, map, &mut walls);
        add_wall(x, y - 1, map, &mut walls);
        add_wall(x, y + 1, map, &mut walls);
    }
    // Now cook shortcuts and eval.
    let mut good_paths = Vec::new();
    let mut paths_by_time: HashMap<usize, Vec<Vec<(i32, i32)>>> = HashMap::new();
    for w in walls {
        let cheat_path = path(map, Some(w));
        let path_len = cheat_path.len() - 1;
        let saved = canon_len - path_len;
        if saved > 0 {
            paths_by_time.entry(saved).or_default().push(cheat_path.clone());
        }
        if saved >= threshold {
            good_paths.push(cheat_path);
            println!("Good shortcut: {w:?}, path_len: {path_len}, saved: {saved}");
        }
    }
    for (saved, paths) in paths_by_time {
        println!("Saving {saved}: {} paths", paths.len());
    }
    good_paths.len()
}


#[test]
fn testerino_eval() {
    let map = Map::parse(TEST_MAP);
    assert_eq!(count_cheats(&map, 2), 14);
}

fn main() {
    let input = read_to_string("day20/input.txt").unwrap();
    let map = Map::parse(&input);
    println!("p1: {}", count_cheats(&map, 100));
}
