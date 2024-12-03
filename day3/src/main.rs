// Day3 wants us to do some parsing...
// Let's try using `nom` for this purpose.
// We choose `nom`, because it defines parsers as rust functions, which is appealing for quickhax.

use nom::{
    branch::alt,
    bytes::{tag, take},
    character::complete::{char, digit1},
    combinator::{map, map_res, value},
    error::VerboseError,
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

fn parse_num(i: &str) -> IResult<&str, i32, VerboseError<&str>> {
    let pos_num = map_res(digit1, |digit_str: &str| digit_str.parse::<i32>());
    let neg_num = map_res(preceded(tag("-"), digit1), |digit_str: &str| {
        digit_str.parse::<i32>().map(|i| -i)
    });
    alt((pos_num, neg_num)).parse(i)
}

#[test]
fn test_num_tag() {
    assert_eq!(parse_num("666").unwrap().1, 666);
    assert_eq!(parse_num("-49").unwrap().1, -49);
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    /// Multiplication op
    Mul(i32, i32),
    /// Do op
    Do,
    /// Don't op
    Dont,
}

fn parse_mul(i: &str) -> IResult<&str, Op, VerboseError<&str>> {
    // A valid multiplication instruction is "mul", "(", Number, ",", Number, ")"
    let parser = preceded(
        tag("mul"),
        delimited(
            char('('),
            separated_pair(parse_num, char(','), parse_num),
            char(')'),
        ),
    );
    map(parser, |(a, b)| Op::Mul(a, b)).parse(i)
}

#[test]
fn test_mul() {
    let txt = r#"mul(2,3)"#;
    let (rem, imul) = parse_mul(txt).unwrap();
    println!("{imul:?}");
    println!("rem: {rem}");
}

fn parse_do(i: &str) -> IResult<&str, Op, VerboseError<&str>> {
    value(Op::Do, tag("do()")).parse(i)
}

fn parse_dont(i: &str) -> IResult<&str, Op, VerboseError<&str>> {
    value(Op::Dont, tag("don't()")).parse(i)
}

#[test]
fn test_do_dont() {
    assert_eq!(parse_do("do()").unwrap().1, Op::Do);
    assert_eq!(parse_dont("don't()").unwrap().1, Op::Dont);
}

/// Parses all the valid [IMultiplication] instructions in the input sequence `i`, discarding the rest.
fn parse_op_sequence(i: &str) -> IResult<&str, Vec<Op>, VerboseError<&str>> {
    let mut ops = Vec::new();
    let mut outer_rem = i;
    let mut parse_op = alt((parse_mul, parse_do, parse_dont));
    // This could probably be written using nom, but I failed (see below for failure mode).
    while !outer_rem.is_empty() {
        if let Ok((rem, op)) = parse_op.parse(outer_rem) {
            outer_rem = rem;
            ops.push(op);
        } else {
            // Snip off a character.
            let (rem, _) = take(1u8).parse(outer_rem)?;
            outer_rem = rem;
        }
    }
    // The below version is my failure with nom.
    // let res = many0(alt((
    //     map(parse_i_mul, |imul| Some(imul)),
    //     value(None, anychar),
    // )))
    // .parse(i)?;
    // let (remainder, muls) = res;
    Ok((outer_rem, ops))
}

/// Sums all muls, obeying any do or don't instructions.
fn sum_ops(ops: &[Op]) -> i32 {
    let mut sum = 0;
    let mut op_do = true;
    for op in ops {
        match op {
            Op::Mul(a, b) => {
                if op_do {
                    sum += a * b;
                }
            }
            Op::Do => op_do = true,
            Op::Dont => op_do = false,
        }
    }
    sum
}

#[test]
fn test_parser() {
    let sequence = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    let (_, parsed_muls) = parse_op_sequence(sequence).unwrap();
    assert_eq!(
        parsed_muls,
        [Op::Mul(2, 4), Op::Mul(5, 5), Op::Mul(11, 8), Op::Mul(8, 5)]
    );
    assert_eq!(sum_ops(&parsed_muls), 161);
}

#[test]
fn test_parser_2() {
    let sequence = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
    let (_, parsed_ops) = parse_op_sequence(sequence).unwrap();
    assert_eq!(
        parsed_ops,
        [
            Op::Mul(2, 4),
            Op::Dont,
            Op::Mul(5, 5),
            Op::Mul(11, 8),
            Op::Do,
            Op::Mul(8, 5)
        ]
    );
    assert_eq!(sum_ops(&parsed_ops), 48);
}

fn main() {
    let input = std::fs::read_to_string("day3/input.txt").unwrap();
    let (_, ops) = parse_op_sequence(&input).unwrap();
    // For part 1 result, you have to remove all Op::Do, Op::Dont from the parsed `ops`.
    let sum = sum_ops(&ops);
    println!("{sum}");
}
