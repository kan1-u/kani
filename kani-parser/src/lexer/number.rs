use crate::alias::{Float, Int};
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{char, digit1, hex_digit1, oct_digit1, one_of};
use nom::combinator::{map_res, opt, recognize};
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

pub fn integer(input: &str) -> IResult<&str, Int> {
    alt((hexadecimal, octal, binary, decimal))(input)
}

pub fn float(input: &str) -> IResult<&str, Float> {
    map_res(float_str, |x| x.replace("_", "").parse())(input)
}

fn hexadecimal(input: &str) -> IResult<&str, Int> {
    map_res(hexadecimal_str, |x| {
        Int::from_str_radix(&x.replace("_", ""), 16)
    })(input)
}

fn octal(input: &str) -> IResult<&str, Int> {
    map_res(octal_str, |x| Int::from_str_radix(&x.replace("_", ""), 8))(input)
}

fn binary(input: &str) -> IResult<&str, Int> {
    map_res(binary_str, |x| Int::from_str_radix(&x.replace("_", ""), 2))(input)
}

fn decimal(input: &str) -> IResult<&str, Int> {
    map_res(decimal_str, |x| {
        Int::from_str_radix(&x.replace("_", ""), 10)
    })(input)
}

fn hexadecimal_str(input: &str) -> IResult<&str, &str> {
    preceded(
        tag_no_case("0x"),
        recognize(many1(terminated(hex_digit1, many0(char('_'))))),
    )(input)
}

fn octal_str(input: &str) -> IResult<&str, &str> {
    preceded(
        tag_no_case("0o"),
        recognize(many1(terminated(oct_digit1, many0(char('_'))))),
    )(input)
}

fn binary_str(input: &str) -> IResult<&str, &str> {
    preceded(
        tag_no_case("0b"),
        recognize(many1(terminated(one_of("01"), many0(char('_'))))),
    )(input)
}

fn decimal_str(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(digit1, many0(char('_')))))(input)
}

fn float_str(input: &str) -> IResult<&str, &str> {
    alt((
        recognize(tuple((
            char('.'),
            decimal_str,
            opt(tuple((one_of("eE"), opt(one_of("+-")), decimal_str))),
        ))),
        recognize(tuple((
            decimal_str,
            opt(preceded(char('.'), decimal_str)),
            one_of("eE"),
            opt(one_of("+-")),
            decimal_str,
        ))),
        recognize(tuple((decimal_str, char('.'), opt(decimal_str)))),
    ))(input)
}
