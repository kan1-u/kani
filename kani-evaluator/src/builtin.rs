use super::object::*;

pub fn builtins() -> Vec<Builtin> {
    vec![print(), len(), head(), tail()]
}

fn print() -> Builtin {
    Builtin::new("print", |arg| match arg {
        Some(Object::String(t)) => {
            println!("{}", t);
            Ok(Object::Null)
        }
        Some(o) => {
            println!("{}", o);
            Ok(Object::Null)
        }
        _ => Err("invalid arguments for print"),
    })
}

fn len() -> Builtin {
    Builtin::new("len", |arg| match arg {
        Some(Object::String(s)) => Ok(Object::Integer(s.len() as i64)),
        Some(Object::Array(arr)) => Ok(Object::Integer(arr.len() as i64)),
        _ => Err("invalid arguments for len"),
    })
}

fn head() -> Builtin {
    Builtin::new("head", |arg| match arg {
        Some(Object::Array(arr)) => match arr.first() {
            Some(x) => Ok(x.clone()),
            None => Err("empty array"),
        },
        _ => Err("invalid arguments for head"),
    })
}

fn tail() -> Builtin {
    Builtin::new("tail", |arg| match arg {
        Some(Object::Array(arr)) => match arr.len() {
            0 => Err("empty array"),
            _ => Ok(Object::Array(Array(arr[1..].into()))),
        },
        _ => Err("invalid arguments for tail"),
    })
}
