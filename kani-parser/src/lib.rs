mod alias;
pub mod ast;
pub mod lexer;
mod macros;
pub mod parser;
pub mod token;

pub use nom;
#[doc(inline)]
pub use parser::program;
