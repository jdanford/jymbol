use gc::Gc;

use crate::{function::Closure, symbol, value::Compound, FnId, Symbol, Value};

impl From<Symbol> for Value {
    fn from(sym: Symbol) -> Self {
        Value::Symbol(sym)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        if b {
            Value::true_()
        } else {
            Value::false_()
        }
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

impl Value {
    pub fn symbol<S: AsRef<str>>(s: S) -> Self {
        Symbol::new(s).into()
    }

    #[must_use]
    pub fn compound(type_: Symbol, values: Vec<Value>) -> Self {
        let compound = Compound { type_, values };
        Value::Compound(Gc::new(compound))
    }

    #[must_use]
    pub fn closure(fn_id: FnId, values: Vec<Value>) -> Self {
        let closure = Closure { fn_id, values };
        Value::Closure(Gc::new(closure))
    }

    pub fn cons(head: Value, tail: Value) -> Self {
        Value::compound(*symbol::CONS, vec![head, tail])
    }

    pub fn list<T: AsRef<[Value]>>(values: T) -> Self {
        let mut list = Value::nil();

        for value in values.as_ref().iter().rev() {
            list = Value::cons(value.clone(), list);
        }

        list
    }

    pub fn quote(value: Value) -> Self {
        Value::compound(*symbol::QUOTE, vec![value])
    }

    pub fn quasiquote(value: Value) -> Self {
        Value::compound(*symbol::QUASIQUOTE, vec![value])
    }

    pub fn unquote(value: Value) -> Self {
        Value::compound(*symbol::UNQUOTE, vec![value])
    }

    pub fn unquote_splicing(value: Value) -> Self {
        Value::compound(*symbol::UNQUOTE_SPLICING, vec![value])
    }
}
