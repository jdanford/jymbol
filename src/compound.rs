use std::fmt::{self, Display, Formatter};

use gc::{Finalize, Trace};

use crate::{Result, Symbol, Value};

#[derive(Clone, PartialEq, PartialOrd, Debug, Trace, Finalize)]
pub struct Compound {
    pub type_: Symbol,
    pub values: Vec<Value>,
}

impl Compound {
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn check_len(&self, expected_len: usize) -> Result<()> {
        let actual_len = self.len();
        if actual_len == expected_len {
            Ok(())
        } else {
            Err(format!("expected {expected_len} values, got {actual_len}"))
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
