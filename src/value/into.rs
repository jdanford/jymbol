use anyhow::anyhow;
use gc::Gc;

use crate::{value::Compound, Error, FnId, Result, Symbol, Value};

impl TryInto<Symbol> for Value {
    type Error = Error;

    fn try_into(self) -> Result<Symbol> {
        Value::as_symbol(&self)
    }
}

impl TryInto<f64> for Value {
    type Error = Error;

    fn try_into(self) -> Result<f64> {
        Value::as_number(&self)
    }
}

impl Value {
    pub fn as_symbol(&self) -> Result<Symbol> {
        if let &Value::Symbol(sym) = self {
            Ok(sym)
        } else {
            Err(anyhow!("expected symbol, got {self}"))
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        if self == &Value::true_() {
            Ok(true)
        } else if self == &Value::false_() {
            Ok(false)
        } else {
            Err(anyhow!("expected true or false, got {self}"))
        }
    }

    pub fn as_number(&self) -> Result<f64> {
        if let &Value::Number(num) = self {
            Ok(num)
        } else {
            Err(anyhow!("expected number, got {self}"))
        }
    }

    pub fn as_string(&self) -> Result<Gc<String>> {
        if let Value::String(s) = self {
            Ok(s.clone())
        } else {
            Err(anyhow!("expected string, got {self}"))
        }
    }

    pub fn as_compound(&self) -> Result<Gc<Compound>> {
        if let Value::Compound(compound) = self {
            Ok(compound.clone())
        } else {
            Err(anyhow!("expected compound, got {self}"))
        }
    }

    pub fn as_native_function(&self) -> Result<FnId> {
        if let &Value::NativeFunction(fn_id) = self {
            Ok(fn_id)
        } else {
            Err(anyhow!("expected native function, got {self}"))
        }
    }
}
