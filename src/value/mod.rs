mod compound;
mod display;
mod from;
mod hash;
mod into;
mod iterator;

use std::cmp::Ordering;

pub use compound::Compound;

use dumpster::{unsync::Gc, Trace};

use crate::{function, symbol, FnId, Symbol};

#[derive(Clone, PartialEq, Debug, Trace)]
pub enum Value {
    Symbol(Symbol),
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

    pub fn is_nil(&self) -> bool {
        self == &Value::nil()
    }

    pub fn is_boolean(&self) -> bool {
        self == &Value::true_() || self == &Value::false_()
    }

    pub fn is_truthy(&self) -> bool {
        *self != Value::false_()
    }

    pub fn type_(&self) -> Symbol {
        match self {
            _ if self.is_nil() => *symbol::NIL,
            _ if self.is_boolean() => *symbol::BOOLEAN,
            Value::Symbol(_) => *symbol::SYMBOL,
            Value::Number(_) => *symbol::NUMBER,
            Value::String(_) => *symbol::STRING,
            Value::Closure(_) => *symbol::FN,
            Value::NativeFunction(_) => *symbol::NATIVE_FN,
            Value::Compound(compound) => compound.type_,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Symbol(a), Value::Symbol(b)) => a.partial_cmp(b),
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            (Value::Compound(a), Value::Compound(b)) if a.type_ == b.type_ => {
                a.values.partial_cmp(&b.values)
            }
            _ => self.type_().partial_cmp(&other.type_()),
        }
    }
}
