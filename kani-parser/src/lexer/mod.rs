mod identifier;
mod illegal;
mod number;
mod operator;
mod punctuation;
mod reserved;
mod string;

use crate::token::*;
use identifier::*;
use illegal::*;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::combinator::into;
use nom::sequence::delimited;
use nom::IResult;
use number::*;
use operator::*;
use punctuation::*;
use reserved::*;
use string::*;

pub fn token(input: &str) -> IResult<&str, Token> {
    delimited(
        multispace0,
        alt((
            operator_token,
            punctuation_token,
            string_token,
            reserved_token,
            identifier_token,
            float_token,
            integer_token,
            illegal_token,
        )),
        multispace0,
    )(input)
}

fn operator_token(input: &str) -> IResult<&str, Token> {
    alt((
        into(equal),
        into(not_equal),
        into(assign),
        into(plus),
        into(minus),
        into(multiply),
        into(divide),
        into(rem),
        into(not),
        into(greater_than_equal),
        into(less_than_equal),
        into(greater_than),
        into(less_than),
        into(at),
        into(dollar),
    ))(input)
}

fn punctuation_token(input: &str) -> IResult<&str, Token> {
    alt((
        into(comma),
        into(semi_colon),
        into(colon),
        into(dot),
        into(pipe),
        into(lparen),
        into(rparen),
        into(lbrace),
        into(rbrace),
        into(lbracket),
        into(rbracket),
    ))(input)
}

fn string_token(input: &str) -> IResult<&str, Token> {
    into(string)(input)
}

fn reserved_token(input: &str) -> IResult<&str, Token> {
    alt((
        into(return_),
        into(if_),
        into(then),
        into(else_),
        into(true_),
        into(false_),
    ))(input)
}

fn identifier_token(input: &str) -> IResult<&str, Token> {
    into(identifier)(input)
}

fn float_token(input: &str) -> IResult<&str, Token> {
    into(float)(input)
}

fn integer_token(input: &str) -> IResult<&str, Token> {
    into(integer)(input)
}

fn illegal_token(input: &str) -> IResult<&str, Token> {
    into(illegal)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier_test() {
        assert_eq!(
            token("abc_123"),
            Ok(("", Identifier("abc_123".into()).into()))
        );
    }

    #[test]
    fn illegal_test() {
        assert_eq!(token("#"), Ok(("", Illegal.into())));
    }

    #[test]
    fn integer_test() {
        assert_eq!(token("12_34"), Ok(("", 1234.into())));
        assert_eq!(token("0xEF_12"), Ok(("", 0xEF12.into())));
        assert_eq!(token("0o67_12"), Ok(("", 0o6712.into())));
        assert_eq!(token("0b01_10"), Ok(("", 0b110.into())));
    }

    #[test]
    fn float_test() {
        assert_eq!(token("12_34.56_78"), Ok(("", 1234.5678.into())));
        assert_eq!(token("1234.56e+78"), Ok(("", 1234.56e+78.into())));
        assert_eq!(token("1234.56e-78"), Ok(("", 1234.56e-78.into())));
    }

    #[test]
    fn operator_test() {
        assert_eq!(token("=="), Ok(("", Equal.into())));
        assert_eq!(token("!="), Ok(("", NotEqual.into())));
        assert_eq!(token("="), Ok(("", Assign.into())));
        assert_eq!(token("+"), Ok(("", Plus.into())));
        assert_eq!(token("-"), Ok(("", Minus.into())));
        assert_eq!(token("*"), Ok(("", Multiply.into())));
        assert_eq!(token("/"), Ok(("", Divide.into())));
        assert_eq!(token("%"), Ok(("", Rem.into())));
        assert_eq!(token("!"), Ok(("", Not.into())));
        assert_eq!(token(">="), Ok(("", GreaterThanEqual.into())));
        assert_eq!(token("<="), Ok(("", LessThanEqual.into())));
        assert_eq!(token(">"), Ok(("", GreaterThan.into())));
        assert_eq!(token("<"), Ok(("", LessThan.into())));
        assert_eq!(token("@"), Ok(("", At.into())));
        assert_eq!(token("$"), Ok(("", Dollar.into())));
    }

    #[test]
    fn punctuation_test() {
        assert_eq!(token(","), Ok(("", Comma.into())));
        assert_eq!(token(";"), Ok(("", SemiColon.into())));
        assert_eq!(token(":"), Ok(("", Colon.into())));
        assert_eq!(token("."), Ok(("", Dot.into())));
        assert_eq!(token("|"), Ok(("", Pipe.into())));
        assert_eq!(token("("), Ok(("", LParenthesis.into())));
        assert_eq!(token(")"), Ok(("", RParenthesis.into())));
        assert_eq!(token("{"), Ok(("", LBrace.into())));
        assert_eq!(token("}"), Ok(("", RBrace.into())));
        assert_eq!(token("["), Ok(("", LBracket.into())));
        assert_eq!(token("]"), Ok(("", RBracket.into())));
    }

    #[test]
    fn reserved_test() {
        assert_eq!(token("return"), Ok(("", Return.into())));
        assert_eq!(token("if"), Ok(("", If.into())));
        assert_eq!(token("then"), Ok(("", Then.into())));
        assert_eq!(token("else"), Ok(("", Else.into())));
        assert_eq!(token("true"), Ok(("", true.into())));
        assert_eq!(token("false"), Ok(("", false.into())));
    }
    #[test]
    fn string_test() {
        assert_eq!(
            token("\"test\\n\\r\\t\\b\\f\\\\\\/\\\"\\u{0A}\""),
            Ok(("", "test\n\r\t\u{08}\u{0C}\\/\"\u{0A}".to_string().into()))
        );
    }
}
