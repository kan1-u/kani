use super::Illegal;
use nom::bytes::complete::take;
use nom::combinator::value;
use nom::IResult;

pub fn illegal(input: &str) -> IResult<&str, Illegal> {
    value(Illegal, take(1usize))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn illegal_test() {
        assert_eq!(illegal("1;"), Ok((";", Illegal)));
    }
}
