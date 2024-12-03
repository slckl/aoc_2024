// Day3 wants us to do some parsing...
// Let's try using `nom` for this purpose.
// We choose `nom`, because it defines parsers as rust functions, which is appealing for quickhax.

use nom::{
    branch::alt,
    bytes::{
        complete::{take_until, take_while},
        tag, take,
    },
    character::complete::{anychar, char, digit1},
    combinator::{cut, map, map_res, value},
    error::VerboseError,
    multi::{many0, many_till},
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

/// A complete multiplication instruction, with `a` and `b` components.
#[derive(Debug, Clone, PartialEq)]
struct IMultiplication {
    a: i32,
    b: i32,
}

impl IMultiplication {
    fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }
}

fn parse_i_mul(i: &str) -> IResult<&str, IMultiplication, VerboseError<&str>> {
    // A valid multiplication instruction is "mul", "(", Number, ",", Number, ")"
    let parser = preceded(
        tag("mul"),
        delimited(
            char('('),
            separated_pair(parse_num, char(','), parse_num),
            char(')'),
        ),
    );
    map(parser, |(a, b)| IMultiplication { a, b }).parse(i)
}

#[test]
fn test_mul() {
    let txt = r#"mul(2,3)"#;
    let (rem, imul) = parse_i_mul(txt).unwrap();
    println!("{imul:?}");
    println!("rem: {rem}");
}

/// Parses all the valid [IMultiplication] instructions in the input sequence `i`, discarding the rest.
fn parse_mul_sequence(i: &str) -> IResult<&str, Vec<IMultiplication>, VerboseError<&str>> {
    let mut muls = Vec::new();
    let mut outer_rem = i;
    // This could probably be written using nom, but I failed (see below for failure mode).
    while !outer_rem.is_empty() {
        if let Ok((rem, mul)) = parse_i_mul(outer_rem) {
            outer_rem = rem;
            muls.push(mul);
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
    Ok((outer_rem, muls))
}

fn sum_muls(muls: &[IMultiplication]) -> i32 {
    muls.iter().map(|mul| mul.a * mul.b).sum()
}

#[test]
fn test_parser() {
    let sequence = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    let (_, parsed_muls) = parse_mul_sequence(sequence).unwrap();
    assert_eq!(
        parsed_muls,
        [
            IMultiplication::new(2, 4),
            IMultiplication::new(5, 5),
            IMultiplication::new(11, 8),
            IMultiplication::new(8, 5)
        ]
    );
    assert_eq!(sum_muls(&parsed_muls), 161);
}

fn main() {
    let input = std::fs::read_to_string("day3/input.txt").unwrap();
    let (_, muls) = parse_mul_sequence(&input).unwrap();
    let sum = sum_muls(&muls);
    println!("{sum}");
}
