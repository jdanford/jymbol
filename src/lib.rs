#![cfg_attr(feature = "cargo-clippy", deny(clippy::all, clippy::pedantic))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::missing_docs_in_private_items, clippy::missing_errors_doc)
)]

mod arity;
mod compiler;
mod env;
mod error;
mod function;
mod iterator;
mod module;
pub mod parser;
pub mod symbol;
mod value;
mod vm;

pub use arity::Arity;
pub use env::Env;
pub use error::{Error, Result};
pub use function::FnId;
pub use iterator::ResultIterator;
pub use module::Module;
pub use symbol::Symbol;
pub use value::Value;
pub use vm::VM;
