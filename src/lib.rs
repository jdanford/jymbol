#![cfg_attr(feature = "cargo-clippy", deny(clippy::all, clippy::pedantic))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::missing_docs_in_private_items, clippy::missing_errors_doc)
)]

mod arity;
pub mod builtin;
mod compound;
mod env;
mod error;
mod function;
mod hash;
mod iterator;
pub mod native;
mod parser;
pub mod read;
pub mod symbol;
mod value;
mod vm;

pub use arity::Arity;
pub use compound::Compound;
pub use env::Env;
pub use error::{Error, Result};
pub use function::Function;
pub use hash::hash;
pub use iterator::{ResultIterator, ValueIterator};
pub use symbol::Symbol;
pub use value::Value;
pub use vm::VM;
