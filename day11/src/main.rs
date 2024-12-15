use std::{collections::HashMap, fs::read_to_string, time::Instant};

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
    Two(i64, i64),
}

/// Splits `n` into `a` and `b`, if `n` has even number of digits.
/// `a` and `b` correspond to left and right side of `n`'s digits, respectively.
/// Any leading 0 remaining in `b` after the split are ignored.
#[inline(always)]
fn split_if_even_n_of_digits(n: i64) -> Option<(i64, i64)> {
    // Count digits
    let digit_count = if n == 0 { 1 } else {
        (n.abs() as f64).log10().floor() as i32 + 1
    };
    
    // Early return if odd number of digits
    if digit_count % 2 != 0 {
        return None;
    }
    
    // Calculate half point
    let half = digit_count / 2;
    
    // Extract right half
    let divisor = 10_i64.pow(half as u32);
    let right = n % divisor;
    
    // Extract left half
    let left = n / divisor;
    
    Some((left, right))
}

#[inline(always)]
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
    let mut result = Vec::with_capacity(stones.len() * 2);
    // Cache computation result for starting stone + computation step.
    // There should be many repeated computations, allowing us to save memory big time.
    // let mut cache: HashMap<(i64, usize), Vec<i64>> = HashMap::new();
    stones
        .into_iter()
        // .into_par_iter()
        // .for_each_with(&mut result, |result, stone| {
        .for_each(|stone| {
            // First, check cache.
            match advance_stone(stone) {
                StoneResult::One(res) => result.push(res),
                StoneResult::Two(a, b) => {
                    result.push(a);
                    result.push(b);
                }
            }
        });

    // stones.reserve(stones.len() * 2);

    // let mut i = 0;
    // while i < stones.len() {
    //     let stone = stones[i];
    //     match advance_stone(stone) {
    //         StoneResult::One(res) => {
    //             stones[i] = res;
    //             i += 1;
    //         }
    //         StoneResult::Two(a, b) => {
    //             stones[i] = a;
    //             stones.insert(i, b);
    //             i += 2;
    //         }
    //     }
    // }
    println!(
        "{iter}: Blinked in {:?}, stones: {}",
        start.elapsed(),
        result.len()
        // stones.len()
    );
    result
    // stones
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

// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;


// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

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
