use gc::Gc;

use crate::{native, symbol, Arity, Function, Symbol, Value};

use super::compound::Compound;

impl From<Symbol> for Value {
    fn from(sym: Symbol) -> Self {
        Value::Symbol(sym)
    }
}

impl From<f64> for Value {
    fn from(num: f64) -> Self {
        Value::Number(num)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(Gc::new(s))
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

impl Value {
    pub fn symbol<S: AsRef<str>>(s: S) -> Value {
        Symbol::new(s).into()
    }

    #[must_use]
    pub fn compound(type_: Symbol, values: Vec<Value>) -> Value {
        let compound = Compound { type_, values };
        Value::Compound(Gc::new(compound))
    }

    pub fn native_function<A: Into<Arity>>(
        f: native::RawFunction,
        arity: A,
        eval_args: bool,
    ) -> Value {
        native::Function::new(f, arity, eval_args).into()
    }

    #[must_use]
    pub fn nil() -> Value {
        (*symbol::NIL).into()
    }

    pub fn cons(head: Value, tail: Value) -> Value {
        Value::compound(*symbol::CONS, vec![head, tail])
    }

    pub fn list<T: AsRef<[Value]>>(values: T) -> Value {
        let mut list = Value::nil();

        for value in values.as_ref().iter().rev() {
            list = Value::cons(value.clone(), list);
        }

        list
    }

    pub fn quote(value: Value) -> Value {
        Value::compound(*symbol::QUOTE, vec![value])
    }

    pub fn quasiquote(value: Value) -> Value {
        Value::compound(*symbol::QUASIQUOTE, vec![value])
    }

    pub fn unquote(value: Value) -> Value {
        Value::compound(*symbol::UNQUOTE, vec![value])
    }

    pub fn unquote_splicing(value: Value) -> Value {
        Value::compound(*symbol::UNQUOTE_SPLICING, vec![value])
    }
}
