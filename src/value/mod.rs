mod compound;
mod display;
mod from;
mod hash;
mod into;
mod iterator;

pub use compound::Compound;
pub use iterator::Iter;

use gc::{Finalize, Gc, Trace};

use crate::{function, symbol, FnId, Symbol};

#[derive(Clone, PartialEq, Debug, Trace, Finalize)]
pub enum Value {
    Blank,
    Symbol(Symbol),
    RestSymbol(Option<Symbol>),
    Number(f64),
    String(Gc<String>),
    Compound(Gc<Compound>),
    Closure(Gc<function::Closure>),
    NativeFunction(FnId),
}

impl Eq for Value {}

impl Value {
    #[must_use]
    pub fn nil() -> Value {
        (*symbol::NIL).into()
    }

    #[must_use]
    pub fn true_() -> Value {
        (*symbol::TRUE).into()
    }

    #[must_use]
    pub fn false_() -> Value {
        (*symbol::FALSE).into()
    }

    pub fn is_truthy(&self) -> bool {
        *self != Value::false_()
    }
}
