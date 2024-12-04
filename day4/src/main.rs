/// Represent text as 2d massive,
/// with line_len x lines dimensions.
pub struct Text2D {
    chars: Vec<char>,
    line_len: i32,
    lines: i32,
}

impl Text2D {
    pub fn new(txt: &str) -> Self {
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

    pub fn at(&self, x: i32, y: i32) -> Option<char> {
        if x >= 0 && x < self.line_len && y >= 0 && y < self.lines {
            Some(self.chars[(x + y * self.line_len) as usize])
        } else {
            None
        }
    }
}

#[cfg(test)]
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
    let text = Text2D::new(TEST_CASE);
    assert_eq!(text.at(0, 0), Some('M'));
    assert_eq!(text.at(1, 1), Some('S'));
    assert_eq!(text.at(9, 9), Some('X'));
    assert_eq!(text.at(0, -1), None);
    assert_eq!(text.at(10, 11), None);
    assert_eq!(text.at(10, 9), None);
}

fn next_letter(cur: char, candidate: char) -> (bool, bool) {
    let (valid, finished) = match cur {
        'X' => (candidate == 'M', false),
        'M' => (candidate == 'A', false),
        // If next letter is S, we have XMAS
        'A' => (false, candidate == 'S'),
        _ => (false, false),
    };
    (valid, finished)
}

fn inner(x: i32, y: i32, dx: i32, dy: i32, txt: &Text2D) -> (i32, i32, bool, bool) {
    let cur = txt.at(x, y).unwrap();
    let tx = x + dx;
    let ty = y + dy;
    let Some(candidate) = txt.at(tx, ty) else {
        return (tx, ty, false, false);
    };
    let (valid, finished) = next_letter(cur, candidate);
    let victory = if finished { "XMAS" } else { "" };
    println!("-- ({dx}, {dy}) {cur} -> {candidate}: ({x}, {y}) -> ({tx}, {ty}) {victory}");
    (tx, ty, valid, finished)
}

fn xmas_loop(x: i32, y: i32, dx: i32, dy: i32, txt: &Text2D, count: &mut usize) {
    let mut tx = x;
    let mut ty = y;
    let mut valid = true;
    while valid {
        let finished;
        (tx, ty, valid, finished) = inner(tx, ty, dx, dy, txt);
        if finished {
            *count += 1;
        }
    }
}

fn check(x: i32, y: i32, txt: &Text2D, count: &mut usize) {
    // TODO
    let ch = txt.at(x, y).unwrap();
    if ch != 'X' {
        return;
    }
    // -1, -1
    xmas_loop(x, y, -1, -1, txt, count);
    // -1, 0
    xmas_loop(x, y, -1, 0, txt, count);
    // -1, 1
    xmas_loop(x, y, -1, 1, txt, count);
    // 0, -1
    xmas_loop(x, y, 0, -1, txt, count);
    // 0, 1
    xmas_loop(x, y, 0, 1, txt, count);
    // 1, -1
    xmas_loop(x, y, 1, -1, txt, count);
    // 1, 0
    xmas_loop(x, y, 1, 0, txt, count);
    // 1, 1
    xmas_loop(x, y, 1, 1, txt, count);
}

fn count_xmas(i: &str) -> usize {
    println!("Humm");
    let text = Text2D::new(i);
    let mut count = 0;
    println!("text.lines: {}", text.lines);
    println!("text.line_len: {}", text.line_len);
    for y in 0..text.lines {
        for x in 0..text.line_len {
            // TODO
            let ch = text.at(x, y).unwrap();
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
fn test_count_xmas() {
    assert_eq!(count_xmas(TEST_CASE), 18);
}

fn main() {
    let text = std::fs::read_to_string("day4/input.txt").unwrap();
    println!("{}", count_xmas(&text));
}
