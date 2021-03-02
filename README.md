# kani

A programming language I created for my learning.

## Features

- Parser written by nom.

```rust
use kani_parser::*;
use kani_parser::ast::*;

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
```

- Simple lazy execution.

```rust
use kani_evaluator::{nom, Evaluator};

let code = "1 + 1";

match Evaluator::new().eval_code(code) {
    Ok(object) => println!("{}", object),
    Err(nom::Err::Error(_)) => println!("Parser error"),
    Err(nom::Err::Failure(_)) => println!("Parser failure"),
    Err(nom::Err::Incomplete(_)) => println!("Incomplete parsing"),
}
```

- All functions are curried and parsed.

```rust
use kani_parser::*;
use kani_parser::ast::*;

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
```

```rust
use kani_parser::*;
use kani_parser::ast::*;

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
```

## Examples

```
map = |f, v|
    if (len(v) == 0)
        then []
        else {
            h = head(v);
            [f(h)] + map(f, tail(v))
        }

reduce = |f, init, v|
    if (len(v) == 0)
        then init
        else {
            i = f(init, head(v));
            reduce(f, i, tail(v))
        }

double = map(|x| x * 2)
sum = reduce(|a, b| a + b, 0)
prod = reduce(|a, b| a * b, 1)

a = double([1, 2, 3, 4, 5])
map(print, a)

a = sum([1, 2, 3, 4, 5])
print(a)

a = prod([1, 2, 3, 4, 5])
print(a)
```

```
map = |f, v| if len(v) == 0 then [] else [f(head(v))] + map(f, tail(v))

people = [{"name": "Sato", "age": 20}, {"name": "Suzuki", "age": 21}]
name = |p| p["name"]
age = |p| p["age"]

a = map(name, people)
print(a)

a = map(age)(people)
print(a)
```

```
map = |f, v| if len(v) == 0 then [] else [f(head(v))] + map(f, tail(v))
range = |a, b| if a == b then [b] else range(a, b - 1) + [b]

fizzbuzz = |x| {
    if (x % 3 == 0) then if (x % 5 == 0) then return "FizzBuzz"
    if (x % 3 == 0) then return "Fizz"
    if (x % 5 == 0) then return "Buzz"
    x
}

print(map(fizzbuzz, range(1, 15)))
```

## Usage

### REPL

```bash
cargo run -p kani-repl
```

### Run code

```bash
cargo run -p kani-cmd -- -c "1 + 1"
```

### Run from file

```bash
cargo run -p kani-cmd -- -f examples/1.kn
```
