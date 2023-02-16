mod compound;
mod display;
mod from;
mod hash;
mod into;
mod iterator;

pub use compound::Compound;
pub use iterator::Iter;

use gc::{Finalize, Gc, Trace};

use crate::{native, Function, Symbol};

#[derive(Clone, PartialEq, PartialOrd, Debug, Trace, Finalize)]
pub enum Value {
    Blank,
    Symbol(Symbol),
    RestSymbol(Option<Symbol>),
    Number(f64),
    String(Gc<String>),
    Compound(Gc<Compound>),
    Function(Gc<Function>),
    NativeFunction(Gc<native::Function>),
}
