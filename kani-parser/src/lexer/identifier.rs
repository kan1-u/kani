use super::Identifier;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1};
use nom::combinator::{map, recognize};
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;

pub fn identifier(input: &str) -> IResult<&str, Identifier> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |x: &str| Identifier(x.to_owned()),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier_test() {
        assert_eq!(
            identifier("abc_123;"),
            Ok((";", Identifier("abc_123".into())))
        );
    }
}
