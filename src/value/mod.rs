mod compound;
mod from;
mod hash;
mod into;
mod iterator;

pub use iterator::Iter;

use std::fmt::{self, Debug, Display, Formatter};

use gc::{Finalize, Gc, Trace};

use crate::{native, Function, Symbol};

pub use self::compound::Compound;

#[derive(Clone, PartialEq, PartialOrd, Debug, Trace, Finalize)]
pub enum Value {
    Blank,
    Symbol(Symbol),
    RestSymbol(Option<Symbol>),
    Number(f64),
    String(Gc<String>),
    Function(Gc<Function>),
    NativeFunction(Gc<native::Function>),
    Compound(Gc<Compound>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Blank => write!(f, "_"),
            Value::Symbol(sym) => Display::fmt(&sym, f),
            Value::RestSymbol(None) => write!(f, "..."),
            Value::RestSymbol(Some(sym)) => write!(f, "{sym}..."),
            Value::Number(num) => {
                if num.fract() == 0.0 {
                    write!(f, "{num:.0}")
                } else {
                    write!(f, "{num}")
                }
            }
            Value::String(s) => Debug::fmt(&s, f),
            Value::Function(fn_) => Display::fmt(&fn_, f),
            Value::NativeFunction(fn_) => Display::fmt(&fn_, f),
            Value::Compound(compound) => Display::fmt(&compound, f),
        }
    }
}
