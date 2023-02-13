use std::fmt::{self, Display, Formatter};

use gc::{Finalize, Trace};

use crate::{symbol, Arity, Result, Symbol, Value};

#[derive(Clone, PartialEq, PartialOrd, Debug, Trace, Finalize)]
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

    pub fn as_checked<const N: usize>(&self, expected_type: Symbol) -> Result<[Value; N]> {
        if self.type_ == expected_type {
            Arity::from(N).check(self.len())?;
            Ok(self.values.clone().try_into().unwrap())
        } else {
            let actual_type = self.type_;
            Err(format!("expected {expected_type}, got {actual_type}"))
        }
    }

    pub fn as_cons(&self) -> Result<(Value, Value)> {
        let values = self.as_checked::<2>(*symbol::CONS)?;
        match values {
            [head, tail] => Ok((head, tail)),
        }
    }
}

impl Display for Compound {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(#{}", self.type_)?;
        for value in &self.values {
            write!(f, " {value}")?;
        }
        write!(f, ")")?;
        Ok(())
    }
}
