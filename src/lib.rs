#![cfg_attr(feature = "cargo-clippy", deny(clippy::all, clippy::pedantic))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::missing_docs_in_private_items, clippy::missing_errors_doc)
)]

mod arity;
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
mod symbol;
mod value;
mod vm;

pub use arity::Arity;
pub use convert::try_as_array;
pub use env::Env;
pub use error::{Error, Result};
pub use expr::Expr;
pub use function::FnId;
pub use instruction::Inst;
pub use iterator::ResultIterator;
pub use module::Module;
pub use symbol::Symbol;
pub use value::Value;
pub use vm::VM;
