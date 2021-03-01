use crate::alias::*;
use crate::{impl_deref, impl_from_enum};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal(Illegal),
    Identifier(Identifier),
    Str(String),
    Int(Int),
    Float(Float),
    Bool(Bool),
    Equal(Equal),
    NotEqual(NotEqual),
    Assign(Assign),
    Plus(Plus),
    Minus(Minus),
    Multiply(Multiply),
    Divide(Divide),
    Rem(Rem),
    Not(Not),
    GreaterThanEqual(GreaterThanEqual),
    LessThanEqual(LessThanEqual),
    GreaterThan(GreaterThan),
    LessThan(LessThan),
    Comma(Comma),
    SemiColon(SemiColon),
    Colon(Colon),
    Dot(Dot),
    Pipe(Pipe),
    LParenthesis(LParenthesis),
    RParenthesis(RParenthesis),
    LBrace(LBrace),
    RBrace(RBrace),
    LBracket(LBracket),
    RBracket(RBracket),
    Return(Return),
    If(If),
    Else(Else),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Illegal;
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);
#[derive(Debug, Clone, PartialEq)]
pub struct Equal;
#[derive(Debug, Clone, PartialEq)]
pub struct NotEqual;
#[derive(Debug, Clone, PartialEq)]
pub struct Assign;
#[derive(Debug, Clone, PartialEq)]
pub struct Plus;
#[derive(Debug, Clone, PartialEq)]
pub struct Minus;
#[derive(Debug, Clone, PartialEq)]
pub struct Multiply;
#[derive(Debug, Clone, PartialEq)]
pub struct Divide;
#[derive(Debug, Clone, PartialEq)]
pub struct Rem;
#[derive(Debug, Clone, PartialEq)]
pub struct Not;
#[derive(Debug, Clone, PartialEq)]
pub struct GreaterThanEqual;
#[derive(Debug, Clone, PartialEq)]
pub struct LessThanEqual;
#[derive(Debug, Clone, PartialEq)]
pub struct GreaterThan;
#[derive(Debug, Clone, PartialEq)]
pub struct LessThan;
#[derive(Debug, Clone, PartialEq)]
pub struct Comma;
#[derive(Debug, Clone, PartialEq)]
pub struct SemiColon;
#[derive(Debug, Clone, PartialEq)]
pub struct Colon;
#[derive(Debug, Clone, PartialEq)]
pub struct Dot;
#[derive(Debug, Clone, PartialEq)]
pub struct Pipe;
#[derive(Debug, Clone, PartialEq)]
pub struct LParenthesis;
#[derive(Debug, Clone, PartialEq)]
pub struct RParenthesis;
#[derive(Debug, Clone, PartialEq)]
pub struct LBrace;
#[derive(Debug, Clone, PartialEq)]
pub struct RBrace;
#[derive(Debug, Clone, PartialEq)]
pub struct LBracket;
#[derive(Debug, Clone, PartialEq)]
pub struct RBracket;
#[derive(Debug, Clone, PartialEq)]
pub struct Return;
#[derive(Debug, Clone, PartialEq)]
pub struct If;
#[derive(Debug, Clone, PartialEq)]
pub struct Else;

impl_from_enum!(Token::Illegal, Illegal);
impl_from_enum!(Token::Identifier, Identifier);
impl_from_enum!(Token::Str, String);
impl_from_enum!(Token::Int, Int);
impl_from_enum!(Token::Float, Float);
impl_from_enum!(Token::Bool, Bool);
impl_from_enum!(Token::Equal, Equal);
impl_from_enum!(Token::NotEqual, NotEqual);
impl_from_enum!(Token::Assign, Assign);
impl_from_enum!(Token::Plus, Plus);
impl_from_enum!(Token::Minus, Minus);
impl_from_enum!(Token::Multiply, Multiply);
impl_from_enum!(Token::Divide, Divide);
impl_from_enum!(Token::Rem, Rem);
impl_from_enum!(Token::Not, Not);
impl_from_enum!(Token::GreaterThanEqual, GreaterThanEqual);
impl_from_enum!(Token::LessThanEqual, LessThanEqual);
impl_from_enum!(Token::GreaterThan, GreaterThan);
impl_from_enum!(Token::LessThan, LessThan);
impl_from_enum!(Token::Comma, Comma);
impl_from_enum!(Token::SemiColon, SemiColon);
impl_from_enum!(Token::Colon, Colon);
impl_from_enum!(Token::Dot, Dot);
impl_from_enum!(Token::Pipe, Pipe);
impl_from_enum!(Token::LParenthesis, LParenthesis);
impl_from_enum!(Token::RParenthesis, RParenthesis);
impl_from_enum!(Token::LBrace, LBrace);
impl_from_enum!(Token::RBrace, RBrace);
impl_from_enum!(Token::LBracket, LBracket);
impl_from_enum!(Token::RBracket, RBracket);
impl_from_enum!(Token::Return, Return);
impl_from_enum!(Token::If, If);
impl_from_enum!(Token::Else, Else);

impl_deref!(Identifier, String);
