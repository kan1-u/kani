pub mod builtin;
pub mod environment;
mod evaluator;
mod macros;
pub mod object;

use environment::Environment;
pub use evaluator::*;
pub use kani_parser;
use kani_parser::ast::Expression;
pub use kani_parser::nom;
use object::Object;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Evaluator {
    env: Rc<RefCell<Environment>>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::default())),
        }
    }

    pub fn eval_code<'a>(
        &mut self,
        code: &'a str,
    ) -> Result<Object, nom::Err<nom::error::Error<&'a str>>> {
        eval_code(code, &self.env)
    }

    pub fn eval_program(&mut self, program: &[Expression]) -> Object {
        eval_expressions(program, &self.env)
    }

    pub fn eval_expression(&mut self, expression: Expression) -> Object {
        eval_expression(expression, &self.env)
    }
}
