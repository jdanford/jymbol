use crate::{symbol, Result, Value};

pub struct Iter<'a> {
    value: &'a Value,
}

impl<'a> Iter<'a> {
    pub fn new(value: &Value) -> Iter {
        Iter { value }
    }

    fn try_next(&mut self) -> Result<Option<&'a Value>> {
        match self.value {
            Value::Compound(cons) if cons.is_cons() => {
                let [head, tail] = cons.as_array()?;
                self.value = tail;
                Ok(Some(head))
            }
            &Value::Symbol(sym) if sym == *symbol::NIL => Ok(None),
            _ => Err(format!("expected cons or nil, got {}", self.value)),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<&'a Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().transpose()
    }
}

impl<'a> Value {
    pub fn iter(&'a self) -> Iter<'a> {
        Iter::new(self)
    }
}
