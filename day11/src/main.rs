use std::{fs::read_to_string, time::Instant};


fn parse(i: &str) -> Vec<i64> {
    let mut out = Vec::new();
    let split = i.split_ascii_whitespace();
    for s in split {
        out.push(s.parse().unwrap());
    }
    out
}

#[cfg(test)]
const TEST_STONES: &str = "0 1 10 99 999";

#[test]
fn test_parse() {
    let parsed = parse(TEST_STONES);
    assert_eq!(parsed, &[0, 1, 10, 99, 999]);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum StoneResult {
    One(i64),
    Two(i64, i64)
}

/// Dumb string implementation, shame on me, but I'm lazy.
fn split_if_even_n_of_digits(n: i64) -> Option<(i64, i64)> {
    let s = n.to_string();
    if s.len() % 2 == 0 {
        let halfesies = s.len() / 2;
        let left = &s[..halfesies];
        let right = &s[halfesies..];
        return Some((left.parse().unwrap(), right.parse().unwrap()));
    }
    None
}

fn advance_stone(stone: i64) -> StoneResult {
    if stone == 0 {
        StoneResult::One(1)
    } else if let Some((left, right)) = split_if_even_n_of_digits(stone) {
        StoneResult::Two(left, right)
    } else {
        StoneResult::One(stone * 2024)
    }
}

fn blink(mut stones: Vec<i64>, iter: usize) -> Vec<i64> {
    let start = Instant::now();
    let mut idx = 0;
    while idx < stones.len() {
        match advance_stone(stones[idx]) {
            StoneResult::One(res) => {
                stones[idx] = res;
                idx += 1;
            }
            StoneResult::Two(a, b) => {
                stones[idx] = a;
                stones.insert(idx + 1, b);
                idx += 2;
            }
        }
    }
    println!("{iter}: Blinked in {:?}, stones: {}", start.elapsed(), stones.len());
    stones
}

#[test]
fn test_advance() {
    let mut start = vec![125, 17];
    start = blink(start, 0);
    assert_eq!(start, &[253000, 1, 7]);
    start = blink(start, 1);
    assert_eq!(start, &[253, 0, 2024, 14168]);
    start = blink(start, 2);
    assert_eq!(start, &[512072, 1, 20, 24, 28676032]);
    start = blink(start, 3);
    assert_eq!(start, &[512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);
}

fn main() {
    let i = read_to_string("day11/input.txt").unwrap();
    let mut stones = parse(&i);
    for idx in 0..25 {
        stones = blink(stones, idx);
    }
    println!("p1: {}", stones.len());
    for idx in 0..50 {
        stones = blink(stones, idx + 25);
    }
    println!("p2: {}", stones.len());

}
