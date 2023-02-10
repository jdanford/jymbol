use std::fmt::{self, Debug, Display, Formatter};

use gc::{Finalize, Gc, Trace};

use crate::{native, symbol, Arity, Compound, Env, Error, Function, Result, Symbol};

#[derive(Clone, PartialEq, PartialOrd, Debug, Trace, Finalize)]
pub enum Value {
    Number(f64),
    Symbol(Symbol),
    String(Gc<String>),
    Env(Gc<Env>),
    Function(Gc<Function>),
    NativeFunction(Gc<native::Function>),
    Compound(Gc<Compound>),
}

impl From<f64> for Value {
    fn from(num: f64) -> Self {
        Value::Number(num)
    }
}

impl From<Symbol> for Value {
    fn from(sym: Symbol) -> Self {
        Value::Symbol(sym)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(Gc::new(s))
    }
}

impl From<Env> for Value {
    fn from(env: Env) -> Self {
        Value::Env(Gc::new(env))
    }
}

impl From<Function> for Value {
    fn from(fn_: Function) -> Self {
        Value::Function(Gc::new(fn_))
    }
}

impl From<native::Function> for Value {
    fn from(fn_: native::Function) -> Self {
        Value::NativeFunction(Gc::new(fn_))
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

impl TryInto<Symbol> for Value {
    type Error = Error;

    fn try_into(self) -> Result<Symbol> {
        match self {
            Value::Symbol(sym) => Ok(sym),
            _ => Err(format!("expected symbol, got {self}")),
        }
    }
}

impl Value {
    pub fn compound(type_: Symbol, values: Vec<Value>) -> Result<Value> {
        let compound = Compound { type_, values };
        Ok(Value::Compound(Gc::new(compound)))
    }

    pub fn function(env: Gc<Env>, params: Vec<Symbol>, body: Value) -> Result<Value> {
        let fn_ = Function::new(env, params, body);
        Ok(Value::Function(Gc::new(fn_)))
    }

    pub fn native_function<A: Into<Arity>>(
        f: native::RawFunction,
        arity: A,
        eval_args: bool,
    ) -> Result<Value> {
        let fn_ = native::Function::new(f, arity, eval_args);
        Ok(Value::NativeFunction(Gc::new(fn_)))
    }

    pub fn cons(head: Value, tail: Value) -> Result<Value> {
        Value::compound(*symbol::CONS, vec![head, tail])
    }

    pub fn list(values: &[Value]) -> Result<Value> {
        let mut list = Value::from(*symbol::NIL);

        for value in values.iter().rev() {
            list = Value::cons(value.clone(), list)?;
        }

        Ok(list)
    }

    pub fn as_string(&self) -> Result<Gc<String>> {
        if let Value::String(s) = self {
            Ok(s.clone())
        } else {
            Err(format!("expected string, got {self}"))
        }
    }

    pub fn as_env(&self) -> Result<Gc<Env>> {
        if let Value::Env(env) = self {
            Ok(env.clone())
        } else {
            Err(format!("expected env, got {self}"))
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

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(num) => {
                if num.fract() == 0.0 {
                    write!(f, "{num:.0}")
                } else {
                    write!(f, "{num}")
                }
            }
            Value::Symbol(sym) => Display::fmt(&sym, f),
            Value::String(s) => Display::fmt(&s, f),
            Value::Env(env) => Display::fmt(&env, f),
            Value::Function(fn_) => Display::fmt(&fn_, f),
            Value::NativeFunction(fn_) => Display::fmt(&fn_, f),
            Value::Compound(compound) => Display::fmt(&compound, f),
        }
    }
}
