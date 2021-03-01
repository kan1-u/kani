use crate::alias::*;
use crate::{impl_deref, impl_from_enum};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Block(Block),
    Assign(Assign),
    Return(Return),
    Identifier(Identifier),
    Literal(Literal),
    Prefix(Prefix),
    Postfix(Postfix),
    Infix(Infix),
    If(If),
    Function(Function),
    Array(Array),
    Hash(Hash),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block(pub Vec<Expression>);

#[derive(Debug, Clone, PartialEq)]
pub struct Assign {
    pub identifier: Identifier,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return(pub Box<Expression>);

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Str(String),
    Int(Int),
    Float(Float),
    Bool(Bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prefix {
    pub operator: PrefixOperator,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Postfix {
    pub operator: PostfixOperator,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Infix {
    pub operator: InfixOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub cond: Box<Expression>,
    pub consequence: Box<Expression>,
    pub alternative: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub param: Option<Identifier>,
    pub body: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array(pub Vec<Expression>);

#[derive(Debug, Clone, PartialEq)]
pub struct Hash(pub Vec<(HashKey, Expression)>);

#[derive(Debug, Clone, PartialEq)]
pub enum HashKey {
    Str(String),
    Int(Int),
    Bool(Bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrefixOperator {
    Plus,
    Minus,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PostfixOperator {
    Call(Argument),
    Index(Index),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument(pub Option<Box<Expression>>);

#[derive(Debug, Clone, PartialEq)]
pub struct Index(pub Box<Expression>);

#[derive(Debug, Clone, PartialEq)]
pub enum InfixOperator {
    Plus,
    Minus,
    Divide,
    Multiply,
    Rem,
    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

impl_from_enum!(Expression::Block, Block);
impl_from_enum!(Expression::Assign, Assign);
impl_from_enum!(Expression::Return, Return);
impl_from_enum!(Expression::Identifier, Identifier);
impl_from_enum!(Expression::Literal, Literal);
impl_from_enum!(Expression::Prefix, Prefix);
impl_from_enum!(Expression::Postfix, Postfix);
impl_from_enum!(Expression::Infix, Infix);
impl_from_enum!(Expression::If, If);
impl_from_enum!(Expression::Function, Function);
impl_from_enum!(Expression::Array, Array);
impl_from_enum!(Expression::Hash, Hash);

impl_from_enum!(PostfixOperator::Call, Argument);
impl_from_enum!(PostfixOperator::Index, Index);

impl_deref!(Block, Vec<Expression>);
impl_deref!(Return, Expression);
impl_deref!(Identifier, String);
impl_deref!(Array, Vec<Expression>);
impl_deref!(Hash, Vec<(HashKey, Expression)>);
impl_deref!(Argument, Option<Box<Expression>>);
impl_deref!(Index, Box<Expression>);

impl Assign {
    pub fn new(identifier: Identifier, expression: Expression) -> Self {
        Self {
            identifier,
            expression: Box::from(expression),
        }
    }
}

impl Prefix {
    pub fn new(operator: PrefixOperator, expression: Expression) -> Self {
        Self {
            operator,
            expression: Box::new(expression),
        }
    }
}

impl Postfix {
    pub fn new(operator: PostfixOperator, expression: Expression) -> Self {
        Self {
            operator,
            expression: Box::new(expression),
        }
    }

    pub fn currying(args: &[Expression], expression: Expression) -> Self {
        match args.len() {
            0 => Self::new(PostfixOperator::Call(Argument(None)), expression),
            1 => Self::new(
                PostfixOperator::Call(Argument(Some(Box::new(args[0].clone())))),
                expression,
            ),
            i => Self::new(
                PostfixOperator::Call(Argument(Some(Box::new(args[i - 1].clone())))),
                Self::currying(&args[..i - 1], expression).into(),
            ),
        }
    }
}

impl Infix {
    pub fn new(operator: InfixOperator, left: Expression, right: Expression) -> Self {
        Self {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl If {
    pub fn new(
        cond: Expression,
        consequence: Box<Expression>,
        alternative: Option<Box<Expression>>,
    ) -> Self {
        Self {
            cond: Box::new(cond),
            consequence,
            alternative,
        }
    }
}

impl Function {
    pub fn new(param: Option<Identifier>, body: Box<Expression>) -> Self {
        Self { param, body }
    }

    pub fn currying(params: &[Identifier], body: Box<Expression>) -> Self {
        match params.len() {
            0 => Self { param: None, body },
            1 => Self::new(Some(params[0].clone()), body),
            _ => Self::new(
                Some(params[0].clone()),
                Box::new(Self::currying(&params[1..], body).into()),
            ),
        }
    }
}
