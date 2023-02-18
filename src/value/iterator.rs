use crate::{symbol, Result, Value};

pub struct Iter {
    value: Value,
}

impl Iter {
    pub fn new(value: Value) -> Iter {
        Iter { value }
    }

    fn try_next(&mut self) -> Result<Option<Value>> {
        match &self.value {
            Value::Compound(cons) if cons.is_cons() => {
                let (head, tail) = cons.as_cons()?;
                self.value = tail;
                Ok(Some(head))
            }
            &Value::Symbol(sym) if sym == *symbol::NIL => Ok(None),
            _ => Err(format!("expected cons or nil, got {}", self.value)),
        }
    }
}

impl Iterator for Iter {
    type Item = Result<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().transpose()
    }
}

impl IntoIterator for Value {
    type Item = Result<Value>;
    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}
