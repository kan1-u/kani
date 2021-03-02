use super::{Else, If, Return, Then};
use crate::alias::Bool;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::IResult;

pub fn return_(input: &str) -> IResult<&str, Return> {
    value(Return, tag("return"))(input)
}

pub fn if_(input: &str) -> IResult<&str, If> {
    value(If, tag("if"))(input)
}

pub fn then(input: &str) -> IResult<&str, Then> {
    value(Then, tag("then"))(input)
}

pub fn else_(input: &str) -> IResult<&str, Else> {
    value(Else, tag("else"))(input)
}

pub fn true_(input: &str) -> IResult<&str, Bool> {
    value(true, tag("true"))(input)
}

pub fn false_(input: &str) -> IResult<&str, Bool> {
    value(false, tag("false"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserved_test() {
        assert_eq!(return_("return;"), Ok((";", Return)));
        assert_eq!(if_("if;"), Ok((";", If)));
        assert_eq!(then("then;"), Ok((";", Then)));
        assert_eq!(else_("else;"), Ok((";", Else)));
        assert_eq!(true_("true;"), Ok((";", true)));
        assert_eq!(false_("false;"), Ok((";", false)));
    }
}
