use gc::Gc;

use crate::{value::Compound, Error, FnId, Result, Symbol, Value};

impl TryInto<Symbol> for Value {
    type Error = Error;

    fn try_into(self) -> Result<Symbol> {
        match self {
            Value::Symbol(sym) => Ok(sym),
            _ => Err(format!("expected symbol, got {self}")),
        }
    }
}

impl TryInto<f64> for Value {
    type Error = Error;

    fn try_into(self) -> Result<f64> {
        match self {
            Value::Number(num) => Ok(num),
            _ => Err(format!("expected number, got {self}")),
        }
    }
}

impl Value {
    pub fn as_string(&self) -> Result<Gc<String>> {
        if let Value::String(s) = self {
            Ok(s.clone())
        } else {
            Err(format!("expected string, got {self}"))
        }
    }

    pub fn as_compound(&self) -> Result<Gc<Compound>> {
        if let Value::Compound(compound) = self {
            Ok(compound.clone())
        } else {
            Err(format!("expected compound, got {self}"))
        }
    }

    pub fn as_native_function(&self) -> Result<FnId> {
        if let &Value::NativeFunction(fn_id) = self {
            Ok(fn_id)
        } else {
            Err(format!("expected native function, got {self}"))
        }
    }
}
