use crate::ast::*;
use crate::lexer::token;
use crate::token::Token;
use crate::verify_token;
use nom::branch::alt;
use nom::combinator::{into, map, map_opt, opt};
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use nom::IResult;

pub fn expression(input: &str) -> IResult<&str, Expression> {
    pratt(Precedence::Lowest)(input)
}

fn expressions(input: &str) -> IResult<&str, Vec<Expression>> {
    separated_list0(verify_token!(Token::Comma(_)), expression)(input)
}

fn pratt(precedence: Precedence) -> impl FnMut(&str) -> IResult<&str, Expression> {
    move |input| {
        let (input, expression) = atom(input)?;
        pratt_to_peek(input, precedence, expression)
    }
}

fn pratt_to_peek(
    input: &str,
    left: Precedence,
    expression: Expression,
) -> IResult<&str, Expression> {
    if left < Precedence::Call {
        if let Ok((input, arguments)) = arguments(input) {
            let expression = Postfix::currying(&arguments, expression);
            return pratt_to_peek(input, left, expression.into());
        }
    }
    if left < Precedence::Index {
        if let Ok((input, index)) = index(input) {
            let expression = Postfix::new(PostfixOperator::Index(index), expression);
            return pratt_to_peek(input, left, expression.into());
        }
    }
    match infix_operator(input) {
        Ok((input, right)) if left < right.0 => {
            let (input, expression) = infix(input, expression, right)?;
            pratt_to_peek(input, left, expression.into())
        }
        _ => Ok((input, expression)),
    }
}

fn atom(input: &str) -> IResult<&str, Expression> {
    alt((
        into(assign),
        into(return_),
        into(literal),
        into(identifier),
        into(prefix),
        into(parenthesis),
        into(array),
        into(hash),
        into(if_),
        into(function),
        into(block),
    ))(input)
}

fn arguments(input: &str) -> IResult<&str, Vec<Expression>> {
    delimited(
        verify_token!(Token::LParenthesis(_)),
        expressions,
        verify_token!(Token::RParenthesis(_)),
    )(input)
}

fn index(input: &str) -> IResult<&str, Index> {
    map(
        delimited(
            verify_token!(Token::LBracket(_)),
            expression,
            verify_token!(Token::RBracket(_)),
        ),
        |x| Index(Box::new(x)),
    )(input)
}

fn infix_operator(input: &str) -> IResult<&str, (Precedence, InfixOperator)> {
    map_opt(token, |t| match t {
        Token::Equal(_) => Some((Precedence::Equals, InfixOperator::Equal)),
        Token::NotEqual(_) => Some((Precedence::Equals, InfixOperator::NotEqual)),
        Token::LessThan(_) => Some((Precedence::LessGreater, InfixOperator::LessThan)),
        Token::LessThanEqual(_) => Some((Precedence::LessGreater, InfixOperator::LessThanEqual)),
        Token::GreaterThan(_) => Some((Precedence::LessGreater, InfixOperator::GreaterThan)),
        Token::GreaterThanEqual(_) => {
            Some((Precedence::LessGreater, InfixOperator::GreaterThanEqual))
        }
        Token::Plus(_) => Some((Precedence::Sum, InfixOperator::Plus)),
        Token::Minus(_) => Some((Precedence::Sum, InfixOperator::Minus)),
        Token::Multiply(_) => Some((Precedence::Product, InfixOperator::Multiply)),
        Token::Divide(_) => Some((Precedence::Product, InfixOperator::Divide)),
        Token::Rem(_) => Some((Precedence::Product, InfixOperator::Rem)),
        _ => None,
    })(input)
}

fn infix(
    input: &str,
    left: Expression,
    (precedence, operator): (Precedence, InfixOperator),
) -> IResult<&str, Infix> {
    let (input, right) = pratt(precedence)(input)?;
    Ok((input, Infix::new(operator, left, right)))
}

fn assign(input: &str) -> IResult<&str, Assign> {
    map(
        tuple((
            map_opt(token, |t| match t {
                Token::Identifier(x) => Some(Identifier(x.0)),
                _ => None,
            }),
            verify_token!(Token::Assign(_)),
            expression,
        )),
        |(ident, _, expr)| Assign::new(ident, expr),
    )(input)
}

fn return_(input: &str) -> IResult<&str, Return> {
    map(
        tuple((verify_token!(Token::Return(_)), expression)),
        |(_, expr)| Return(Box::new(expr)),
    )(input)
}

fn literal(input: &str) -> IResult<&str, Literal> {
    map_opt(token, |t| match t {
        Token::Str(x) => Some(Literal::Str(x)),
        Token::Int(x) => Some(Literal::Int(x)),
        Token::Float(x) => Some(Literal::Float(x)),
        Token::Bool(x) => Some(Literal::Bool(x)),
        _ => None,
    })(input)
}

fn identifier(input: &str) -> IResult<&str, Identifier> {
    map_opt(token, |t| match t {
        Token::Identifier(x) => Some(Identifier(x.0)),
        _ => None,
    })(input)
}

fn prefix(input: &str) -> IResult<&str, Prefix> {
    let (input, operator) = map_opt(token, |t| match t {
        Token::Plus(_) => Some(PrefixOperator::Plus),
        Token::Minus(_) => Some(PrefixOperator::Minus),
        Token::Not(_) => Some(PrefixOperator::Not),
        _ => None,
    })(input)?;
    let (input, expression) = atom(input)?;
    Ok((input, Prefix::new(operator, expression)))
}

fn parenthesis(input: &str) -> IResult<&str, Expression> {
    delimited(
        verify_token!(Token::LParenthesis(_)),
        expression,
        verify_token!(Token::RParenthesis(_)),
    )(input)
}

fn array(input: &str) -> IResult<&str, Array> {
    map(
        delimited(
            verify_token!(Token::LBracket(_)),
            expressions,
            verify_token!(Token::RBracket(_)),
        ),
        |x| Array(x),
    )(input)
}

fn hash(input: &str) -> IResult<&str, Hash> {
    map(
        delimited(
            verify_token!(Token::LBrace(_)),
            separated_list0(
                verify_token!(Token::Comma(_)),
                separated_pair(hash_key, verify_token!(Token::Colon(_)), expression),
            ),
            verify_token!(Token::RBrace(_)),
        ),
        |x| Hash(x),
    )(input)
}

fn hash_key(input: &str) -> IResult<&str, HashKey> {
    map_opt(token, |t| match t {
        Token::Str(x) => Some(HashKey::Str(x)),
        Token::Int(x) => Some(HashKey::Int(x)),
        Token::Bool(x) => Some(HashKey::Bool(x)),
        _ => None,
    })(input)
}

fn if_(input: &str) -> IResult<&str, If> {
    map(
        tuple((
            verify_token!(Token::If(_)),
            expression,
            expression,
            opt(preceded(verify_token!(Token::Else(_)), expression)),
        )),
        |(_, expr, block, _else)| If::new(expr, Box::new(block), _else.map(|e| e.into())),
    )(input)
}

fn function(input: &str) -> IResult<&str, Function> {
    map(
        tuple((
            verify_token!(Token::Pipe(_)),
            separated_list0(verify_token!(Token::Comma(_)), identifier),
            verify_token!(Token::Pipe(_)),
            expression,
        )),
        |(_, expr, _, block)| Function::currying(&expr, Box::new(block)),
    )(input)
}

pub fn block(input: &str) -> IResult<&str, Block> {
    map(
        delimited(
            verify_token!(Token::LBrace(_)),
            separated_list0(verify_token!(Token::SemiColon(_)), expression),
            verify_token!(Token::RBrace(_)),
        ),
        |x| Block(x),
    )(input)
}
