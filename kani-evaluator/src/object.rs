use super::environment::Environment;
use crate::{impl_deref, impl_from_enum};
use kani_parser::ast::{Expression, HashKey, Identifier, Literal};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::hash;
use std::ops::{Add, Div, Mul, Neg, Not, Rem, Sub};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Array(Array),
    Hash(Hash),
    Function(Function),
    Builtin(Builtin),
    Null,
    Return(Return),
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array(pub Vec<Object>);

#[derive(Debug, Clone, PartialEq)]
pub struct Hash(pub HashMap<Object, Object>);

#[derive(Debug, Clone, PartialEq)]
pub struct Return(pub Box<Object>);

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub param: Option<Identifier>,
    pub body: Expression,
    pub env: Rc<RefCell<Environment>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Builtin {
    pub name: String,
    pub function: BuiltinFunction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionType {
    Function(Function),
    Builtin(Builtin),
}

pub type BuiltinFunction = fn(Option<Object>) -> Result<Object, &'static str>;

impl Object {
    pub fn is_returned(&self) -> bool {
        matches!(*self, Object::Return(_))
    }

    pub fn returned(self) -> Self {
        match self {
            Self::Return(Return(o)) => *o,
            o => o,
        }
    }

    pub fn integer(self) -> Result<i64, String> {
        match self {
            Self::Integer(i) => Ok(i),
            Self::Error(e) => Err(e),
            o => Err(format!("{} is not an integer", o)),
        }
    }

    pub fn boolean(self) -> Result<bool, String> {
        match self {
            Self::Boolean(b) => Ok(b),
            Self::Error(e) => Err(e),
            o => Err(format!("{} is not an boolean", o)),
        }
    }

    pub fn function(self) -> Result<FunctionType, String> {
        match self {
            Self::Function(f) => Ok(FunctionType::Function(f)),
            Self::Builtin(b) => Ok(FunctionType::Builtin(b)),
            Self::Error(e) => Err(e),
            o => Err(format!("{} is not an function", o)),
        }
    }

    pub fn literal(self) -> Result<Literal, String> {
        match self {
            Self::Integer(i) => Ok(Literal::Int(i)),
            Self::Float(f) => Ok(Literal::Float(f)),
            Self::Boolean(b) => Ok(Literal::Bool(b)),
            Self::String(s) => Ok(Literal::Str(s)),
            Self::Error(e) => Err(e),
            o => Err(format!("{} is not an literal", o)),
        }
    }

    pub fn hash_key(self) -> Result<HashKey, String> {
        match self {
            Self::Integer(i) => Ok(HashKey::Int(i)),
            Self::Boolean(b) => Ok(HashKey::Bool(b)),
            Self::String(s) => Ok(HashKey::Str(s)),
            Self::Error(e) => Err(e),
            o => Err(format!("{} is not an hash-key", o)),
        }
    }

    pub fn positive(self) -> Self {
        match self {
            Self::Integer(i) => Self::Integer(i),
            Self::Float(f) => Self::Float(f),
            o => Self::Error(format!("unsupported operation `+`: {:?}", o)),
        }
    }

    pub fn modulo(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Integer(l % r),
            (Self::Float(l), Self::Float(r)) => Self::Float(l % r),
            (Self::Error(s), _) | (_, Self::Error(s)) => Self::Error(s),
            (l, r) => Self::Error(format!("unsupported operations `%`: {:?} and {:?}", l, r)),
        }
    }

    pub fn greater_than(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Boolean(l > r),
            (Self::Float(l), Self::Float(r)) => Self::Boolean(l > r),
            (Self::String(l), Self::String(r)) => Self::Boolean(l > r),
            (Self::Error(s), _) | (_, Self::Error(s)) => Self::Error(s),
            (l, r) => Self::Error(format!("unsupported operations `>`: {:?} and {:?}", l, r)),
        }
    }

    pub fn greater_than_equal(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Boolean(l >= r),
            (Self::Float(l), Self::Float(r)) => Self::Boolean(l >= r),
            (Self::String(l), Self::String(r)) => Self::Boolean(l >= r),
            (Self::Error(s), _) | (_, Self::Error(s)) => Self::Error(s),
            (l, r) => Self::Error(format!("unsupported operations `>=`: {:?} and {:?}", l, r)),
        }
    }

    pub fn less_than(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Boolean(l < r),
            (Self::Float(l), Self::Float(r)) => Self::Boolean(l < r),
            (Self::String(l), Self::String(r)) => Self::Boolean(l < r),
            (Self::Error(s), _) | (_, Self::Error(s)) => Self::Error(s),
            (l, r) => Self::Error(format!("unsupported operations `<`: {:?} and {:?}", l, r)),
        }
    }

    pub fn less_than_equal(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Boolean(l <= r),
            (Self::Float(l), Self::Float(r)) => Self::Boolean(l <= r),
            (Self::String(l), Self::String(r)) => Self::Boolean(l <= r),
            (Self::Error(s), _) | (_, Self::Error(s)) => Self::Error(s),
            (l, r) => Self::Error(format!("unsupported operations `<=`: {:?} and {:?}", l, r)),
        }
    }
}

impl Function {
    pub fn new(param: Option<Identifier>, body: Expression, env: Rc<RefCell<Environment>>) -> Self {
        Self { param, body, env }
    }
}

impl Builtin {
    pub fn new(name: &str, function: BuiltinFunction) -> Self {
        Self {
            name: name.to_string(),
            function,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(i) => i.fmt(f),
            Self::Float(x) => x.fmt(f),
            Self::Boolean(b) => b.fmt(f),
            Self::String(s) => s.fmt(f),
            Self::Array(a) => a.fmt(f),
            Self::Hash(h) => h.fmt(f),
            Self::Function(x) => x.fmt(f),
            Self::Builtin(b) => b.fmt(f),
            Self::Null => write!(f, "null"),
            Self::Return(o) => o.fmt(f),
            Self::Error(s) => write!(f, "Error: {}", s),
        }
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        string.push('[');
        string.push_str(
            &self
                .iter()
                .map(|o| format!("{}", o))
                .collect::<Vec<_>>()
                .join(", "),
        );
        string.push(']');
        write!(f, "{}", string)
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        string.push('{');
        string.push_str(
            &self
                .iter()
                .map(|(k, v)| format!("{} : {}", k, v))
                .collect::<Vec<_>>()
                .join(", "),
        );
        string.push('}');
        write!(f, "{}", string)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[function]")
    }
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[built-in function: {}]", self.name)
    }
}

impl Eq for Object {}

impl hash::Hash for Object {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        match *self {
            Self::Integer(ref i) => i.hash(state),
            Self::Boolean(ref b) => b.hash(state),
            Self::String(ref s) => s.hash(state),
            _ => "".hash(state),
        }
    }
}

impl From<HashKey> for Object {
    fn from(key: HashKey) -> Self {
        match key {
            HashKey::Int(i) => Self::Integer(i),
            HashKey::Bool(b) => Self::Boolean(b),
            HashKey::Str(s) => Self::String(s),
        }
    }
}

impl_from_enum!(Object::Array, Array);
impl_from_enum!(Object::Function, Function);
impl_from_enum!(Object::Builtin, Builtin);
impl_from_enum!(Object::Hash, Hash);
impl_from_enum!(Object::Return, Return);

impl_deref!(Array, Vec<Object>);
impl_deref!(Hash, HashMap<Object, Object>);
impl_deref!(Return, Box<Object>);

impl Not for Object {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Integer(i) => Self::Integer(!i),
            Self::Boolean(b) => Self::Boolean(!b),
            o => Self::Error(format!("unsupported operation `!`: {:?}", o)),
        }
    }
}

impl Neg for Object {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(i) => Self::Integer(-i),
            Self::Float(f) => Self::Float(-f),
            o => Self::Error(format!("unsupported operation `-`: {:?}", o)),
        }
    }
}

impl Add for Object {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Integer(l + r),
            (Self::Float(l), Self::Float(r)) => Self::Float(l + r),
            (Self::String(l), Self::String(r)) => Self::String(l + &r),
            (Self::Array(Array(l)), Self::Array(Array(r))) => Self::Array(Array([l, r].concat())),
            (Self::Error(s), _) | (_, Self::Error(s)) => Self::Error(s),
            (l, r) => Self::Error(format!("unsupported operation `+`: {:?} and {:?}", l, r)),
        }
    }
}

impl Sub for Object {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Integer(l - r),
            (Self::Float(l), Self::Float(r)) => Self::Float(l - r),
            (Self::Error(s), _) | (_, Self::Error(s)) => Self::Error(s),
            (l, r) => Self::Error(format!("unsupported operation `-`: {:?} and {:?}", l, r)),
        }
    }
}

impl Mul for Object {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Integer(l * r),
            (Self::Float(l), Self::Float(r)) => Self::Float(l * r),
            (Self::Error(s), _) | (_, Self::Error(s)) => Self::Error(s),
            (l, r) => Self::Error(format!("unsupported operation `*`: {:?} and {:?}", l, r)),
        }
    }
}

impl Div for Object {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Integer(l / r),
            (Self::Float(l), Self::Float(r)) => Self::Float(l / r),
            (Self::Error(s), _) | (_, Self::Error(s)) => Self::Error(s),
            (l, r) => Self::Error(format!("unsupported operation `/`: {:?} and {:?}", l, r)),
        }
    }
}

impl Rem for Object {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => Self::Integer(l % r),
            (Self::Float(l), Self::Float(r)) => Self::Float(l % r),
            (Self::Error(s), _) | (_, Self::Error(s)) => Self::Error(s),
            (l, r) => Self::Error(format!("unsupported operation `%`: {:?} and {:?}", l, r)),
        }
    }
}
