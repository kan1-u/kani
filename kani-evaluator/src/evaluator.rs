use crate::environment::*;
use crate::nom;
use crate::object::*;
use kani_parser::ast::{
    self, Argument, Assign, Block, Expression, HashKey, Identifier, If, Index, Infix,
    InfixOperator, Literal, Postfix, PostfixOperator, Prefix, PrefixOperator,
};
use kani_parser::program;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn eval_code<'a>(
    code: &'a str,
    env: &Rc<RefCell<Environment>>,
) -> Result<Object, nom::Err<nom::error::Error<&'a str>>> {
    program(code).map(|(_, program)| eval_expressions(&program, env))
}

pub fn eval_expressions(expressions: &[Expression], env: &Rc<RefCell<Environment>>) -> Object {
    match expressions.len() {
        0 => Object::Null,
        1 => eval_expression(expressions[0].clone(), env),
        _ => {
            let object = eval_expression(expressions[0].clone(), env);
            if object.is_returned() {
                object
            } else {
                eval_expressions(&expressions[1..], env)
            }
        }
    }
}

pub fn eval_expression(expression: Expression, env: &Rc<RefCell<Environment>>) -> Object {
    match expression {
        Expression::Block(b) => eval_block(b, env),
        Expression::Assign(a) => eval_assign(a, env),
        Expression::Return(r) => eval_return(r, env),
        Expression::Identifier(i) => eval_identifier(i, env),
        Expression::Literal(l) => eval_literal(l),
        Expression::Prefix(p) => eval_prefix(p, env),
        Expression::Postfix(p) => eval_postfix(p, env),
        Expression::Infix(i) => eval_infix(i, env),
        Expression::If(i) => eval_if(i, env),
        Expression::Function(f) => eval_function(f, env),
        Expression::Array(e) => eval_array(e, env),
        Expression::Hash(h) => eval_hash(h, env),
    }
}

fn eval_block(Block(block): Block, env: &Rc<RefCell<Environment>>) -> Object {
    let child = Environment::from(Rc::clone(env));
    eval_expressions(&block, &Rc::new(RefCell::new(child)))
}

fn eval_assign(
    Assign {
        identifier: Identifier(name),
        expression,
    }: Assign,
    env: &Rc<RefCell<Environment>>,
) -> Object {
    let object = eval_expression(*expression, env);
    env.borrow_mut().set(&name, object.clone());
    object
}

fn eval_return(ast::Return(expression): ast::Return, env: &Rc<RefCell<Environment>>) -> Object {
    Object::Return(Return(Box::new(eval_expression(*expression, env))))
}

fn eval_identifier(Identifier(name): Identifier, env: &Rc<RefCell<Environment>>) -> Object {
    match env.borrow().get(&name) {
        Some(o) => o,
        None => Object::Error(format!("identifier not found: {}", name)),
    }
}

fn eval_literal(literal: Literal) -> Object {
    match literal {
        Literal::Int(i) => Object::Integer(i),
        Literal::Float(f) => Object::Float(f),
        Literal::Bool(b) => Object::Boolean(b),
        Literal::Str(s) => Object::String(s),
    }
}

fn eval_hash_key(literal: HashKey) -> Object {
    match literal {
        HashKey::Int(i) => Object::Integer(i),
        HashKey::Bool(b) => Object::Boolean(b),
        HashKey::Str(s) => Object::String(s),
    }
}

fn eval_prefix(
    Prefix {
        operator,
        expression,
    }: Prefix,
    env: &Rc<RefCell<Environment>>,
) -> Object {
    let object = eval_expression(*expression, env);
    match operator {
        PrefixOperator::Not => !object,
        PrefixOperator::Plus => object.positive(),
        PrefixOperator::Minus => -object,
    }
}

fn eval_postfix(
    Postfix {
        operator,
        expression,
    }: Postfix,
    env: &Rc<RefCell<Environment>>,
) -> Object {
    match operator {
        PostfixOperator::Call(arguments) => eval_call(*expression, arguments, env),
        PostfixOperator::Index(Index(index)) => eval_index(*expression, *index, env),
    }
}

fn eval_infix(
    Infix {
        operator,
        left,
        right,
    }: Infix,
    env: &Rc<RefCell<Environment>>,
) -> Object {
    let left = eval_expression(*left, env);
    let right = eval_expression(*right, env);
    match operator {
        InfixOperator::Plus => left + right,
        InfixOperator::Minus => left - right,
        InfixOperator::Multiply => left * right,
        InfixOperator::Divide => left / right,
        InfixOperator::Rem => left % right,
        InfixOperator::Equal => Object::Boolean(left == right),
        InfixOperator::NotEqual => Object::Boolean(left != right),
        InfixOperator::GreaterThan => left.greater_than(right),
        InfixOperator::GreaterThanEqual => left.greater_than_equal(right),
        InfixOperator::LessThan => left.less_than(right),
        InfixOperator::LessThanEqual => left.less_than_equal(right),
    }
}

fn eval_if(
    If {
        cond,
        consequence,
        alternative,
    }: If,
    env: &Rc<RefCell<Environment>>,
) -> Object {
    let object = eval_expression(*cond, env);
    match object.boolean() {
        Ok(b) => {
            if b {
                eval_expression(*consequence, env)
            } else {
                match alternative {
                    Some(s) => eval_expression(*s, env),
                    None => Object::Null,
                }
            }
        }
        Err(e) => Object::Error(e),
    }
}

fn eval_function(
    ast::Function { param, body }: ast::Function,
    env: &Rc<RefCell<Environment>>,
) -> Object {
    Object::Function(Function::new(param, *body, Rc::clone(env)))
}

fn eval_call(expression: Expression, arg: Argument, env: &Rc<RefCell<Environment>>) -> Object {
    let object = eval_expression(expression, env);
    match object.function() {
        Ok(FunctionType::Function(f)) => eval_function_call(arg, f, env),
        Ok(FunctionType::Builtin(f)) => eval_builtin_call(arg, f, env),
        Err(e) => Object::Error(e),
    }
}

fn eval_function_call(arg: Argument, function: Function, env: &Rc<RefCell<Environment>>) -> Object {
    let mut child = Environment::from(Rc::clone(&function.env));
    if let Some(arg) = &*arg {
        let arg = eval_expression(*arg.clone(), env);
        if let Some(Identifier(name)) = function.param {
            child.set(&name, arg);
        }
    }
    let object = eval_expression(function.body, &Rc::new(RefCell::new(child)));
    object.returned()
}

fn eval_builtin_call(
    arg: Argument,
    Builtin { function, .. }: Builtin,
    env: &Rc<RefCell<Environment>>,
) -> Object {
    let arg = arg.as_ref().map(|a| eval_expression(*a.clone(), env));
    match function(arg) {
        Ok(o) => o,
        Err(s) => Object::Error(s.to_string()),
    }
}

fn eval_array(arr: ast::Array, env: &Rc<RefCell<Environment>>) -> Object {
    let arr = arr
        .iter()
        .map(|e| eval_expression(e.clone(), env))
        .collect();
    Object::Array(Array(arr))
}

fn eval_hash(hash: ast::Hash, env: &Rc<RefCell<Environment>>) -> Object {
    let mut hashmap = HashMap::new();
    for (k, v) in hash.iter() {
        let key = eval_hash_key(k.clone());
        let value = eval_expression(v.clone(), env);
        hashmap.insert(key, value);
    }
    Object::Hash(Hash(hashmap))
}

fn eval_index(target: Expression, index: Expression, env: &Rc<RefCell<Environment>>) -> Object {
    let target = eval_expression(target, env);
    let index = eval_expression(index, env);
    match target {
        Object::Array(arr) => match index.integer() {
            Ok(i) => arr.get(i as usize).unwrap_or(&Object::Null).clone(),
            Err(e) => Object::Error(e),
        },
        Object::Hash(Hash(hash)) => match index.hash_key() {
            Ok(k) => hash.get(&k.into()).unwrap_or(&Object::Null).clone(),
            Err(e) => Object::Error(e),
        },
        o => Object::Error(format!("unexpected index target: {}", o)),
    }
}
