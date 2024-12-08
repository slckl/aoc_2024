use std::{collections::{HashMap, HashSet}, fs::read_to_string};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Object {
    Empty,
    Antenna(char),
    Antinode(char),
}

#[derive(Debug, PartialEq)]
struct Map {
    objects: Vec<Object>,
    width: i32,
    height: i32,
}

impl Map {
    pub fn parse(i: &str) -> Self {
        let mut objects = Vec::with_capacity(i.len());
        let mut height = 0i32;
        let mut width = 0i32;
        let lines = i.lines();
        for (y, line) in lines.enumerate() {
            width = line.len() as i32;
            height += 1;
            for (x, ch) in line.chars().enumerate() {
                let obj = match ch {
                    '.' => Object::Empty,
                    ch => Object::Antenna(ch),
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

    pub fn at(&self, x: i32, y: i32) -> Option<Object> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(self.objects[self.idx(x, y)])
        } else {
            None
        }
    }
}

#[cfg(test)]
const TEST_MAP: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

#[test]
fn test_parse() {
    let map = Map::parse(TEST_MAP);
    assert_eq!(map.at(0, 0), Some(Object::Empty));
    assert_eq!(map.at(8, 1), Some(Object::Antenna('0')));
    assert_eq!(map.at(6, 5), Some(Object::Antenna('A')));
}

fn comp_antinodes(map: &Map) -> Vec<(i32, i32)> {
    let mut nodes = Vec::new();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for x in 0..map.width {
        for y in 0..map.height {
            let obj = map.at(x, y).unwrap();
            if let Object::Antenna(ch) = obj {
                antennas.entry(ch).or_default().push((x, y));
            }
        }
    }
    // For every antenna type, check every antenna against every other antenna.
    for (_antenna_ty, positions) in antennas {
        for (idx, pos_1) in positions.iter().enumerate() {
            if idx + 1 == positions.len() {
                continue;
            }
            for pos_2 in &positions[idx + 1..] {
                // You can always draw a line between two points.
                // But what kind of line?
                let dx = pos_2.0 - pos_1.0;
                let dy = pos_2.1 - pos_1.1;
                // let ratio = dx as f32 / dy as f32;
                // Antinode has the same distance to the closest antenna
                // as the antennas have between themselves.
                let anode_1 = (pos_2.0 + dx, pos_2.1 + dy);
                let anode_2 = (pos_1.0 - dx, pos_1.1 - dy);
                if map.at(anode_1.0, anode_1.1).is_some() {
                    nodes.push(anode_1);
                }
                if map.at(anode_2.0, anode_2.1).is_some() {
                    nodes.push(anode_2);
                }
            }
        }
    }

    nodes
}

#[test]
fn test_anodes() {
    let map = Map::parse(TEST_MAP);
    let a_nodes = comp_antinodes(&map);
    println!("{a_nodes:?}");
}

fn main() {
    let input = read_to_string("day8/input.txt").unwrap();
    let map = Map::parse(&input);
    let a_nodes: HashSet<_> = comp_antinodes(&map).into_iter().collect();
    println!("p1: {}", a_nodes.len());
}
