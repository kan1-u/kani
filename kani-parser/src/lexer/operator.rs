use super::{
    Assign, At, Divide, Dollar, Equal, GreaterThan, GreaterThanEqual, LessThan, LessThanEqual,
    Minus, Multiply, Not, NotEqual, Plus, Rem,
};
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::IResult;

pub fn equal(input: &str) -> IResult<&str, Equal> {
    value(Equal, tag("=="))(input)
}

pub fn not_equal(input: &str) -> IResult<&str, NotEqual> {
    value(NotEqual, tag("!="))(input)
}

pub fn assign(input: &str) -> IResult<&str, Assign> {
    value(Assign, tag("="))(input)
}

pub fn plus(input: &str) -> IResult<&str, Plus> {
    value(Plus, tag("+"))(input)
}

pub fn minus(input: &str) -> IResult<&str, Minus> {
    value(Minus, tag("-"))(input)
}

pub fn multiply(input: &str) -> IResult<&str, Multiply> {
    value(Multiply, tag("*"))(input)
}

pub fn divide(input: &str) -> IResult<&str, Divide> {
    value(Divide, tag("/"))(input)
}

pub fn rem(input: &str) -> IResult<&str, Rem> {
    value(Rem, tag("%"))(input)
}

pub fn not(input: &str) -> IResult<&str, Not> {
    value(Not, tag("!"))(input)
}

pub fn greater_than_equal(input: &str) -> IResult<&str, GreaterThanEqual> {
    value(GreaterThanEqual, tag(">="))(input)
}

pub fn less_than_equal(input: &str) -> IResult<&str, LessThanEqual> {
    value(LessThanEqual, tag("<="))(input)
}

pub fn greater_than(input: &str) -> IResult<&str, GreaterThan> {
    value(GreaterThan, tag(">"))(input)
}

pub fn less_than(input: &str) -> IResult<&str, LessThan> {
    value(LessThan, tag("<"))(input)
}

pub fn at(input: &str) -> IResult<&str, At> {
    value(At, tag("@"))(input)
}

pub fn dollar(input: &str) -> IResult<&str, Dollar> {
    value(Dollar, tag("$"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_test() {
        assert_eq!(equal("==;"), Ok((";", Equal)));
        assert_eq!(not_equal("!=;"), Ok((";", NotEqual)));
        assert_eq!(assign("=;"), Ok((";", Assign)));
        assert_eq!(plus("+;"), Ok((";", Plus)));
        assert_eq!(minus("-;"), Ok((";", Minus)));
        assert_eq!(multiply("*;"), Ok((";", Multiply)));
        assert_eq!(divide("/;"), Ok((";", Divide)));
        assert_eq!(rem("%;"), Ok((";", Rem)));
        assert_eq!(not("!;"), Ok((";", Not)));
        assert_eq!(greater_than_equal(">=;"), Ok((";", GreaterThanEqual)));
        assert_eq!(less_than_equal("<=;"), Ok((";", LessThanEqual)));
        assert_eq!(greater_than(">;"), Ok((";", GreaterThan)));
        assert_eq!(less_than("<;"), Ok((";", LessThan)));
        assert_eq!(at("@;"), Ok((";", At)));
        assert_eq!(dollar("$;"), Ok((";", Dollar)));
    }
}
