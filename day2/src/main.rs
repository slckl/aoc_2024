use std::{fs::File, io::{BufRead, BufReader}};


#[derive(PartialEq, Debug, Copy, Clone)]
enum Direction {
    Increasing,
    Decreasing
}

fn are_levels_safe(levels: &[u32]) -> bool {
    // Levels must all be either increasing or decreasing.
    // Delta between levels MUST be: 1 <= level <= 3.
    println!("--- Checking safety of {levels:?}");
    let mut previous_level = levels[0];
    let mut previous_direction = None;
    for lvl in &levels[1..] {
        println!("lvl: {lvl}, previous_level: {previous_level}, previous_direction: {previous_direction:?}");
        let lvl = *lvl;
        let delta = previous_level.abs_diff(lvl);
        if delta < 1 || delta > 3 {
            return false;
        }
        if let Some(prev_dir) = previous_direction {
            if lvl > previous_level && prev_dir == Direction::Decreasing {
                return false;
            } else if lvl < previous_level && prev_dir == Direction::Increasing {
                return false;
            }
        } else if lvl > previous_level {
            previous_direction = Some(Direction::Increasing);
        } else {
            previous_direction = Some(Direction::Decreasing);
        }
        previous_level = lvl;
    }

    true
}

#[test]
fn test_levels() {
    let levels = vec![
        (vec![7, 6, 4, 2, 1], true),
        (vec![1, 2, 7, 8, 9], false),
        (vec![9, 7, 6, 2, 1], false),
        (vec![1, 3, 2, 4, 5], false),
        (vec![8, 6, 4, 4, 1], false),
        (vec![1, 3, 6, 7, 9], true),
    ];

    for (case, expected) in levels {
        let safety = are_levels_safe(&case);
        assert_eq!(safety, expected, "levels: {case:?}");
    }
}

fn main() {
    let path = "day2/input.txt";
    let buf_read = BufReader::new(File::open(path).unwrap());
    let mut safe_levels = 0u32;
    for line in buf_read.lines() {
        let line = line.unwrap();
        // Parse line into levels report.
        let report: Vec<u32> = line.split_ascii_whitespace().map(|chunk| chunk.parse().unwrap()).collect();
        if are_levels_safe(&report) {
            safe_levels += 1;
        }
    }
    println!("Safe reports: {safe_levels}");
}
