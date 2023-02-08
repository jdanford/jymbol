#![cfg_attr(feature = "cargo-clippy", deny(clippy::all, clippy::pedantic))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::missing_docs_in_private_items, clippy::missing_errors_doc)
)]

mod compound;
mod env;
mod error;
mod hash;
mod iterator;
mod parser;
mod read;
mod symbol;
mod value;
mod vm;

pub use compound::Compound;
pub use env::Env;
pub use error::{Error, Result};
pub use hash::hash;
pub use iterator::ValueIterator;
pub use read::read;
pub use symbol::Symbol;
pub use value::Value;
pub use vm::VM;
