#![cfg_attr(feature = "cargo-clippy", deny(clippy::all, clippy::pedantic))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::missing_docs_in_private_items, clippy::missing_errors_doc)
)]

mod apply;
mod arity;
pub mod builtin;
mod env;
mod error;
mod function;
mod iterator;
pub mod native;
pub mod parser;
pub mod symbol;
pub mod unify;
mod value;
mod vm;

pub use arity::Arity;
pub use env::Env;
pub use error::{Error, Result};
pub use function::Function;
pub use iterator::ResultIterator;
pub use symbol::Symbol;
pub use value::Value;
pub use vm::VM;
