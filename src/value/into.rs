use gc::Gc;

use crate::{native, value::Compound, Error, Function, Result, Symbol, Value};

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

    pub fn as_function(&self) -> Result<Gc<Function>> {
        if let Value::Function(fn_) = self {
            Ok(fn_.clone())
        } else {
            Err(format!("expected function, got {self}"))
        }
    }

    pub fn as_native_function(&self) -> Result<Gc<native::Function>> {
        if let Value::NativeFunction(fn_) = self {
            Ok(fn_.clone())
        } else {
            Err(format!("expected native function, got {self}"))
        }
    }
}
