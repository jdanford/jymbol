#![cfg_attr(feature = "cargo-clippy", deny(clippy::all, clippy::pedantic))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::missing_docs_in_private_items, clippy::missing_errors_doc)
)]

extern crate chumsky;
extern crate im;
extern crate lazy_static;
extern crate logos;
extern crate symbol_table;

mod check;
mod env;
mod error;
mod hash;
mod heap;
mod list;
mod pack;
mod parser;
mod read;
mod reference;
mod symbol;
mod value;
mod vm;

pub use check::{check_count, check_type};
pub use env::Env;
pub use error::{Error, Result};
pub use heap::Heap;
pub use list::ListIterator;
pub use reference::Ref;
pub use symbol::Symbol;
pub use value::Value;
pub use vm::VM;
