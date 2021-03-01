use super::{
    Colon, Comma, Dot, LBrace, LBracket, LParenthesis, Pipe, RBrace, RBracket, RParenthesis,
    SemiColon,
};
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::IResult;

pub fn comma(input: &str) -> IResult<&str, Comma> {
    value(Comma, tag(","))(input)
}

pub fn semi_colon(input: &str) -> IResult<&str, SemiColon> {
    value(SemiColon, tag(";"))(input)
}

pub fn colon(input: &str) -> IResult<&str, Colon> {
    value(Colon, tag(":"))(input)
}

pub fn dot(input: &str) -> IResult<&str, Dot> {
    value(Dot, tag("."))(input)
}

pub fn pipe(input: &str) -> IResult<&str, Pipe> {
    value(Pipe, tag("|"))(input)
}

pub fn lparen(input: &str) -> IResult<&str, LParenthesis> {
    value(LParenthesis, tag("("))(input)
}

pub fn rparen(input: &str) -> IResult<&str, RParenthesis> {
    value(RParenthesis, tag(")"))(input)
}

pub fn lbrace(input: &str) -> IResult<&str, LBrace> {
    value(LBrace, tag("{"))(input)
}

pub fn rbrace(input: &str) -> IResult<&str, RBrace> {
    value(RBrace, tag("}"))(input)
}

pub fn lbracket(input: &str) -> IResult<&str, LBracket> {
    value(LBracket, tag("["))(input)
}

pub fn rbracket(input: &str) -> IResult<&str, RBracket> {
    value(RBracket, tag("]"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn punctuation_test() {
        assert_eq!(comma(",;"), Ok((";", Comma)));
        assert_eq!(semi_colon(";;"), Ok((";", SemiColon)));
        assert_eq!(colon(":;"), Ok((";", Colon)));
        assert_eq!(dot(".;"), Ok((";", Dot)));
        assert_eq!(pipe("|;"), Ok((";", Pipe)));
        assert_eq!(lparen("(;"), Ok((";", LParenthesis)));
        assert_eq!(rparen(");"), Ok((";", RParenthesis)));
        assert_eq!(lbrace("{;"), Ok((";", LBrace)));
        assert_eq!(rbrace("};"), Ok((";", RBrace)));
        assert_eq!(lbracket("[;"), Ok((";", LBracket)));
        assert_eq!(rbracket("];"), Ok((";", RBracket)));
    }
}
