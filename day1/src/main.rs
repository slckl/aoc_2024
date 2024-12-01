use std::{
    collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path
};

fn total_distance(a: &mut [u32], b: &mut [u32]) -> u32 {
    // Sort.
    a.sort();
    b.sort();

    let mut total = 0u32;
    for (a_elem, b_elem) in a.iter().zip(b.iter()) {
        let distance = a_elem.abs_diff(*b_elem);
        total += distance;
    }
    total
}

#[test]
fn test_distance_11() {
    let mut a = vec![3, 4, 2, 1, 3, 3];
    let mut b = vec![4, 3, 5, 3, 9, 3];

    let total = total_distance(&mut a, &mut b);
    assert_eq!(total, 11);
}

fn read_input(path: impl AsRef<Path>) -> (Vec<u32>, Vec<u32>) {
    let path = path.as_ref();
    let mut a = Vec::new();
    let mut b = Vec::new();

    let buf_read = BufReader::new(File::open(path).unwrap());

    for line in buf_read.lines() {
        let line = line.unwrap();
        // Line is <a_elem> <b_elem>
        let mut line_iter = line.split_ascii_whitespace();
        a.push(line_iter.next().unwrap().parse().unwrap());
        b.push(line_iter.next().unwrap().parse().unwrap());
    }

    (a, b)
}

fn similarity(a: &[u32], b: &[u32]) -> u32 {
    // Count the number of times a number appears in the second list.
    let mut counts_in_b: HashMap<u32, u32> = HashMap::new();
    for b in b {
        *counts_in_b.entry(*b).or_default() += 1;
    }

    // Go through a and compute total "similarity".
    let mut sim = 0u32;
    for a in a {
        let count = counts_in_b.get(a).copied().unwrap_or_default();
        sim += *a * count;
    }
    sim
}

#[test]
fn test_similarity_31() {
    let a = [3, 4, 2, 1, 3, 3];
    let b = [4, 3, 5, 3, 9, 3];
    assert_eq!(similarity(&a, &b), 31);
}

fn main() {
    let (mut a, mut b) = read_input("input.txt");
    let distance = total_distance(&mut a, &mut b);
    println!("distance: {distance}");
    let similarity = similarity(&a, &b);
    println!("similarity: {similarity}");
}
