use crate::{symbol, Result, Value};

#[allow(clippy::module_name_repetitions)]
pub struct ValueIterator {
    value: Value,
}

impl ValueIterator {
    pub fn new(value: Value) -> ValueIterator {
        ValueIterator { value }
    }

    fn try_next(&mut self) -> Result<Option<Value>> {
        match &self.value {
            Value::Compound(cons) if cons.type_ == *symbol::CONS => {
                cons.check_len(2)?;
                let head = cons.values[0].clone();
                let tail = cons.values[1].clone();
                self.value = tail;
                Ok(Some(head))
            }
            Value::Symbol(sym) if *sym == *symbol::NIL => Ok(None),
            _ => Err(format!("expected cons or nil, got {:?}", self.value)),
        }
    }
}

impl Iterator for ValueIterator {
    type Item = Result<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().transpose()
    }
}

impl IntoIterator for Value {
    type Item = Result<Value>;
    type IntoIter = ValueIterator;

    fn into_iter(self) -> Self::IntoIter {
        ValueIterator::new(self)
    }
}
