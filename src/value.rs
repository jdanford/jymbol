use std::fmt::{self, Debug, Display, Formatter};

use gc::{Finalize, Gc, Trace};

use crate::{symbol, Compound, Env, Error, Result, Symbol};

#[derive(Clone, PartialEq, PartialOrd, Debug, Trace, Finalize)]
pub enum Value {
    Number(f64),
    Symbol(Symbol),
    String(Gc<String>),
    Env(Gc<Env>),
    Compound(Gc<Compound>),
}

impl Value {
    pub fn compound(type_: Symbol, values: Vec<Value>) -> Result<Value> {
        let compound = Compound { type_, values };
        Ok(Value::Compound(Gc::new(compound)))
    }

    pub fn cons(head: Value, tail: Value) -> Result<Value> {
        Value::compound(*symbol::CONS, vec![head, tail])
    }

    pub fn list(values: Vec<Value>) -> Result<Value> {
        let mut list = Value::from(*symbol::NIL);

        for value in values.into_iter().rev() {
            list = Value::cons(value, list)?;
        }

        Ok(list)
    }

    pub fn as_string(&self) -> Result<Gc<String>> {
        if let Value::String(string) = self {
            Ok(string.clone())
        } else {
            Err(format!("expected string, got {self:?}"))
        }
    }

    pub fn as_env(&self) -> Result<Gc<Env>> {
        if let Value::Env(env) = self {
            Ok(env.clone())
        } else {
            Err(format!("expected env, got {self:?}"))
        }
    }

    pub fn as_compound(&self) -> Result<Gc<Compound>> {
        if let Value::Compound(compound) = self {
            Ok(compound.clone())
        } else {
            Err(format!("expected compound, got {self:?}"))
        }
    }
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

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::String(Gc::new(string))
    }
}

impl From<Env> for Value {
    fn from(env: Env) -> Self {
        Value::Env(Gc::new(env))
    }
}

impl TryInto<f64> for Value {
    type Error = Error;

    fn try_into(self) -> Result<f64> {
        match self {
            Value::Number(n) => Ok(n),
            _ => Err(format!("expected number, got {self:?}")),
        }
    }
}

impl TryInto<Symbol> for Value {
    type Error = Error;

    fn try_into(self) -> Result<Symbol> {
        match self {
            Value::Symbol(symbol) => Ok(symbol),
            _ => Err(format!("expected symbol, got {self:?}")),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{n:.0}")
                } else {
                    write!(f, "{n}")
                }
            }
            Value::Symbol(symbol) => Display::fmt(&symbol, f),
            Value::String(string) => Display::fmt(&string, f),
            Value::Compound(compound) => Display::fmt(&compound, f),
            Value::Env(env) => Display::fmt(&env, f),
        }
    }
}
