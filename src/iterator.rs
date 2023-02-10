use std::result;

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
            _ => Err(format!("expected cons or nil, got {}", self.value)),
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

// TODO: use generic implementation once stabilized
#[allow(clippy::module_name_repetitions)]
pub trait ResultIterator<T, E>: Iterator<Item = result::Result<T, E>> + Sized {
    fn try_collect(self) -> result::Result<Vec<T>, E> {
        let mut values = Vec::<T>::new();
        for result in self {
            values.push(result?);
        }

        Ok(values)
    }
}

impl<T, E, I: Iterator<Item = result::Result<T, E>>> ResultIterator<T, E> for I {}
