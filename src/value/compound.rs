use anyhow::anyhow;
use dumpster::Trace;

use crate::{Result, Symbol, Value, symbol, try_as_array};

#[derive(Clone, PartialEq, Debug, Trace)]
pub struct Compound {
    pub type_: Symbol,
    pub values: Vec<Value>,
}

impl Compound {
    pub fn new(type_: Symbol, values: Vec<Value>) -> Self {
        Compound { type_, values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn has_type(&self, type_: Symbol) -> bool {
        self.type_ == type_
    }

    pub fn is_cons(&self) -> bool {
        self.has_type(*symbol::CONS)
    }

    pub fn is_quote(&self) -> bool {
        self.has_type(*symbol::QUOTE)
    }

    pub fn is_quasiquote(&self) -> bool {
        self.has_type(*symbol::QUASIQUOTE)
    }

    pub fn is_unquote(&self) -> bool {
        self.has_type(*symbol::UNQUOTE)
    }

    pub fn is_unquote_splicing(&self) -> bool {
        self.has_type(*symbol::UNQUOTE_SPLICING)
    }

    pub fn as_array<const N: usize>(&self) -> Result<&[Value; N]> {
        try_as_array(&self.values)
    }

    pub fn as_checked_array<const N: usize>(&self, expected_type: Symbol) -> Result<&[Value; N]> {
        let actual_type = self.type_;
        if actual_type == expected_type {
            self.as_array()
        } else {
            Err(anyhow!("expected {expected_type}, got {actual_type}"))
        }
    }

    pub fn as_cons(&self) -> Result<(Value, Value)> {
        let [head_ref, tail_ref] = self.as_checked_array(*symbol::CONS)?;
        Ok((head_ref.clone(), tail_ref.clone()))
    }
}
