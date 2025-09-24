#![deny(clippy::pedantic)]
#![allow(clippy::missing_docs_in_private_items, clippy::missing_errors_doc)]

mod arity;
mod builtin;
mod compiler;
mod convert;
mod env;
mod error;
mod expr;
mod function;
mod instruction;
mod iterator;
mod module;
mod parser;
mod special;
mod symbol;
mod value;
mod vm;

pub use arity::Arity;
pub use convert::try_as_array;
pub use env::Env;
pub use error::{Error, Result};
pub use expr::Expr;
pub use function::FnId;
pub use instruction::{Inst, op};
pub use iterator::ResultIterator;
pub use module::Module;
pub use symbol::Symbol;
pub use value::Value;
pub use vm::VM;
