mod expression;

use crate::ast::Expression;
pub use expression::expression;
use expression::root;
use nom::combinator::all_consuming;
use nom::multi::many0;
use nom::IResult;

pub fn program(input: &str) -> IResult<&str, Vec<Expression>> {
    all_consuming(many0(root))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::parser::Expression;

    #[test]
    fn pratt_test() {
        assert_eq!(
            program("1 + 2 * 3"),
            Ok((
                "",
                vec![Expression::Infix(Infix {
                    operator: InfixOperator::Plus,
                    left: Box::new(Expression::Literal(Literal::Int(1))),
                    right: Box::new(Expression::Infix(Infix {
                        operator: InfixOperator::Multiply,
                        left: Box::new(Expression::Literal(Literal::Int(2))),
                        right: Box::new(Expression::Literal(Literal::Int(3)))
                    }))
                })]
            ))
        );
        assert_eq!(
            program("4 / 2 - 1"),
            Ok((
                "",
                vec![Expression::Infix(Infix {
                    operator: InfixOperator::Minus,
                    left: Box::new(Expression::Infix(Infix {
                        operator: InfixOperator::Divide,
                        left: Box::new(Expression::Literal(Literal::Int(4))),
                        right: Box::new(Expression::Literal(Literal::Int(2)))
                    })),
                    right: Box::new(Expression::Literal(Literal::Int(1))),
                })]
            ))
        );
    }

    #[test]
    fn function_test() {
        let add = vec![Expression::Assign(Assign {
            identifier: Identifier("add".to_string()),
            expression: Box::new(Expression::Function(Function {
                param: Some(Identifier("a".to_string())),
                body: Box::new(Expression::Function(Function {
                    param: Some(Identifier("b".to_string())),
                    body: Box::new(Expression::Infix(Infix {
                        operator: InfixOperator::Plus,
                        left: Box::new(Expression::Identifier(Identifier("a".to_string()))),
                        right: Box::new(Expression::Identifier(Identifier("b".to_string()))),
                    })),
                })),
            })),
        })];
        assert_eq!(program("add = |a, b| a + b"), Ok(("", add.clone())));
        assert_eq!(program("add = |a| |b| a + b"), Ok(("", add)));
    }

    #[test]
    fn call_test() {
        let add = vec![Expression::Postfix(Postfix {
            operator: PostfixOperator::Call(Argument(Some(Box::new(Expression::Literal(
                Literal::Int(2),
            ))))),
            expression: Box::new(Expression::Postfix(Postfix {
                operator: PostfixOperator::Call(Argument(Some(Box::new(Expression::Literal(
                    Literal::Int(1),
                ))))),
                expression: Box::new(Expression::Identifier(Identifier("add".to_string()))),
            })),
        })];
        assert_eq!(program("add(1, 2)"), Ok(("", add.clone())));
        assert_eq!(program("add(1)(2)"), Ok(("", add)));
    }
}
