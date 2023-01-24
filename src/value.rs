use crate::{Error, Ref, Result, Symbol};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Number(f64),
    Symbol(Symbol),
    Ref(Ref),
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Number(f)
    }
}

impl From<Symbol> for Value {
    fn from(symbol: Symbol) -> Self {
        Value::Symbol(symbol)
    }
}

impl From<Ref> for Value {
    fn from(ref_: Ref) -> Self {
        Value::Ref(ref_)
    }
}

impl TryFrom<Value> for f64 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Number(n) => Ok(n),
            _ => Err(format!("expected number, got {:?}", value)),
        }
    }
}

impl TryFrom<Value> for Symbol {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Symbol(symbol) => Ok(symbol),
            _ => Err(format!("expected symbol, got {:?}", value)),
        }
    }
}

impl TryFrom<Value> for Ref {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Ref(ref_) => Ok(ref_),
            _ => Err(format!("expected ref, got {:?}", value)),
        }
    }
}
