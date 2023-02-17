use gc::{Finalize, Trace};

use crate::{symbol, Arity, Result, Symbol, Value};

#[derive(Clone, PartialEq, Debug, Trace, Finalize)]
pub struct Compound {
    pub type_: Symbol,
    pub values: Vec<Value>,
}

impl Compound {
    #[must_use]
    pub fn new(type_: Symbol, values: Vec<Value>) -> Self {
        Compound { type_, values }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn is_cons(&self) -> bool {
        self.type_ == *symbol::CONS
    }

    pub fn is_quote(&self) -> bool {
        self.type_ == *symbol::QUOTE
    }

    pub fn is_quasiquote(&self) -> bool {
        self.type_ == *symbol::QUASIQUOTE
    }

    pub fn is_unquote(&self) -> bool {
        self.type_ == *symbol::UNQUOTE
    }

    pub fn is_unquote_splicing(&self) -> bool {
        self.type_ == *symbol::UNQUOTE_SPLICING
    }

    pub fn as_array<const N: usize>(&self) -> Result<[Value; N]> {
        Arity::from(N).check(self.len())?;
        Ok(self.values.clone().try_into().unwrap())
    }

    pub fn as_checked_array<const N: usize>(&self, expected_type: Symbol) -> Result<[Value; N]> {
        let actual_type = self.type_;
        if actual_type == expected_type {
            self.as_array()
        } else {
            Err(format!("expected {expected_type}, got {actual_type}"))
        }
    }

    pub fn as_cons(&self) -> Result<(Value, Value)> {
        let [head, tail] = self.as_checked_array::<2>(*symbol::CONS)?;
        Ok((head, tail))
    }
}
