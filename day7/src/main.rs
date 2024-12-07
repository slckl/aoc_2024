use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
struct Equation {
    value: i64,
    numbers: Vec<i64>,
    // operators: Vec<Op>
}

fn parse(i: &str) -> Vec<Equation> {
    let mut eqs = Vec::new();
    for line in i.lines() {
        let mut numbers = line.split_ascii_whitespace();
        // First is value:
        let value = numbers.next().unwrap();
        let value = value[..value.len() - 1].parse().unwrap();
        let mut eq = Equation {
            value,
            numbers: vec![],
            // operators: vec![]
        };
        // Rest are numbers
        for num in numbers {
            eq.numbers.push(num.parse().unwrap());
        }
        eqs.push(eq);
    }
    eqs
}

#[cfg(test)]
const TEST_DOC: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

#[test]
fn test_parse() {
    let parsed = parse(TEST_DOC);
    assert_eq!(
        parsed[0],
        Equation {
            value: 190,
            numbers: vec![10, 19],
            // operators: vec![],
        }
    );
    assert_eq!(
        parsed[8],
        Equation {
            value: 292,
            numbers: vec![11, 6, 16, 20],
            // operators: vec![]
        }
    );
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Mul,
    Conc,
}

fn value(nums: &[i64], ops: &[Op]) -> i64 {
    let mut value = nums[0];

    for (op_idx, op) in ops.iter().enumerate() {
        value = match op {
            Op::Add => value + nums[op_idx + 1],
            Op::Mul => value * nums[op_idx + 1],
            Op::Conc => format!("{value}{}", nums[op_idx + 1]).parse().unwrap(),
        };
    }
    value
}

#[test]
fn test_value() {
    assert_eq!(value(&[10, 19], &[Op::Add]), 29);
    assert_eq!(value(&[10, 19], &[Op::Mul]), 190);
    assert_eq!(value(&[81, 40, 27], &[Op::Add, Op::Mul]), 3267);
    assert_eq!(value(&[81, 40, 27], &[Op::Mul, Op::Add]), 3267);
}

fn permutations(elements: &[Op], length: usize) -> Vec<Vec<Op>> {
    let mut result = Vec::new();
    let base = elements.len();
    let total = base.pow(length as u32);

    for i in 0..total {
        let mut current = Vec::with_capacity(length);
        let mut n = i;

        for _current_idx in 0..length {
            let pos = n % base;
            current.push(elements[pos]);
            n /= base;
        }

        result.push(current);
    }

    result
}

fn can_be_true(eq: &Equation, elements: &[Op]) -> bool {
    // Generate all permutations of operator chains using the 2 ops.
    // If at least one produces the value specified by the equation, return true.
    let op_len = eq.numbers.len() - 1;
    let permutations = permutations(elements, op_len);
    for op_seq in permutations {
        if value(&eq.numbers, &op_seq) == eq.value {
            return true;
        }
    }
    false
}

#[test]
fn test_can_be_true() {
    fn can_be_true_2_op(eq: &Equation) -> bool {
        let elements = &[Op::Add, Op::Mul];
        can_be_true(eq, elements)
    }

    let parsed = parse(TEST_DOC);
    assert!(can_be_true_2_op(&parsed[0]));
    assert!(can_be_true_2_op(&parsed[1]));
    assert!(!can_be_true_2_op(&parsed[2]));
    assert!(!can_be_true_2_op(&parsed[3]));
    assert!(!can_be_true_2_op(&parsed[4]));
    assert!(!can_be_true_2_op(&parsed[5]));
    assert!(!can_be_true_2_op(&parsed[6]));
    assert!(!can_be_true_2_op(&parsed[7]));
    assert!(can_be_true_2_op(&parsed[8]));
}

fn p1(i: &str) -> i64 {
    let parsed = parse(i);
    let elements = &[Op::Add, Op::Mul];
    parsed
        .iter()
        .filter_map(|eq| {
            if can_be_true(eq, elements) {
                Some(eq.value)
            } else {
                None
            }
        })
        .sum()
}

fn p2(i: &str) -> i64 {
    let parsed = parse(i);
    let elements = &[Op::Add, Op::Mul, Op::Conc];
    parsed
        .iter()
        .filter_map(|eq| {
            if can_be_true(eq, elements) {
                Some(eq.value)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let i = read_to_string("day7/input.txt").unwrap();
    println!("p1: {}", p1(&i));
    println!("p2: {}", p2(&i));
}
