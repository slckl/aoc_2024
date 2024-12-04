// Day 4 wants us to find XMAS in any directions.
// For every char for every line:
//  Check if it's X
//  For every direction check if it's M
//  For every found M, check all directions if it's A
//  For every found A, check all directions if it's S
//  For every XMAS, increment total count.

/// Represent text as 2d massive,
/// with line_len x lines dimensions.
pub struct Text2D {
    chars: Vec<char>,
    line_len: i32,
    lines: i32,
}

impl Text2D {
    pub fn from_str(txt: &str) -> Self {
        let mut chars = Vec::with_capacity(txt.len());
        let mut lines_count = 0usize;
        let mut line_len = 0usize;
        let lines = txt.lines();
        for line in lines {
            line_len = line.len();
            lines_count += 1;
            for ch in line.chars() {
                chars.push(ch);
            }
        }
        Text2D {
            chars,
            line_len: line_len as i32,
            lines: lines_count as i32,
        }
    }

    pub fn at(&self, x: i32, y: i32) -> char {
        self.chars[(x + y * self.line_len) as usize]
    }
}

const TEST_CASE: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

#[test]
fn test_indexing() {
    let text = Text2D::from_str(TEST_CASE);
    assert_eq!(text.at(0, 0), 'M');
    assert_eq!(text.at(1, 1), 'S');
    assert_eq!(text.at(9, 9), 'X');
}

fn next_letter(cur: char, candidate: char) -> (bool, bool) {
    let (valid, finished) = match cur {
        'X' => (candidate == 'M', false),
        'M' => (candidate == 'A', false),
        'A' => (false, candidate == 'S'),
        // 'S' => (false, true),
        _ => (false, false),
    };
    (valid, finished)
}

fn inner(x: i32, y: i32, dx: i32, dy: i32, txt: &Text2D) -> (i32, i32, bool, bool) {
    let cur = txt.at(x, y);
    let tx = x + dx;
    let ty = y + dy;
    let candidate = txt.at(tx, ty);
    let (valid, finished) = next_letter(cur, candidate);
    let victory = if finished { "XMAS" } else { "" };
    println!("-- {cur} -> {candidate}: ({x}, {y}) -> ({tx}, {ty}) {victory}");
    (tx, ty, valid, finished)
}

fn check(x: i32, y: i32, txt: &Text2D, count: &mut usize) {
    let ch = txt.at(x, y);
    if ch != 'X' {
        return;
    }
    // -1, -1
    let mut tx = x;
    let mut ty = y;
    let mut valid = true;
    while valid && tx > 0 && ty > 0 {
        let finished;
        (tx, ty, valid, finished) = inner(tx, ty, -1, -1, txt);
        if finished {
            *count += 1;
        } 
    }
    // -1, 0
    let mut tx = x;
    let mut ty = y;
    let mut valid = true;
    while valid && tx > 0 {
        let finished;
        (tx, ty, valid, finished) = inner(tx, ty, -1, 0, txt);
        if finished {
            *count += 1;
        } 
    }
    // -1, 1
    let mut tx = x;
    let mut ty = y;
    let mut valid = true;
    while valid && tx > 0 && ty + 1 < txt.lines {
        let finished;
        (tx, ty, valid, finished) = inner(tx, ty, -1, 1, txt);
        if finished {
            *count += 1;
        } 
    }
    // 0, -1
    let mut tx = x;
    let mut ty = y;
    let mut valid = true;
    while valid && ty > 0 {
        let finished;
        (tx, ty, valid, finished) = inner(tx, ty, 0, -1, txt);
        if finished {
            *count += 1;
        } 
    }
    // 0, 1
    let mut tx = x;
    let mut ty = y;
    let mut valid = true;
    while valid && ty + 1 < txt.lines {
        let finished;
        (tx, ty, valid, finished) = inner(tx, ty, 0, 1, txt);
        if finished {
            *count += 1;
        } 
    }
    // 1, -1
    let mut tx = x;
    let mut ty = y;
    let mut valid = true;
    while valid && tx + 1 < txt.line_len && ty > 0 {
        let finished;
        (tx, ty, valid, finished) = inner(tx, ty, 1, -1, txt);
        if finished {
            *count += 1;
        } 
    }
    // 1, 0
    let mut tx = x;
    let mut ty = y;
    let mut valid = true;
    while valid && tx + 1 < txt.line_len {
        let finished;
        (tx, ty, valid, finished) = inner(tx, ty, 1, 0, txt);
        if finished {
            *count += 1;
        } 
    }
    // 1, 1
    let mut tx = x;
    let mut ty = y;
    let mut valid = true;
    while valid && tx + 1 < txt.line_len && ty + 1 < txt.lines {
        let finished;
        (tx, ty, valid, finished) = inner(tx, ty, 1, 1, txt);
        if finished {
            *count += 1;
        } 
    }
}

fn count_xmas(i: &str) -> usize {
    println!("Humm");
    let text = Text2D::from_str(i);
    let mut count = 0;
    println!("text.lines: {}", text.lines);
    println!("text.line_len: {}", text.line_len);
    for y in 0..text.lines {
        for x in 0..text.line_len {
            let ch = text.at(x, y);
            println!("({x}, {y}): {ch}");
            if ch != 'X' {
                continue;
            }
            // Check all valid directions for next letter.
            check(x, y, &text, &mut count);
        }
    }
    count
}

#[test]
fn test_count() {
    assert_eq!(count_xmas(TEST_CASE), 18);
}

fn main() {
    let text = std::fs::read_to_string("day4/input.txt").unwrap();
    println!("{}", count_xmas(&text));
}
