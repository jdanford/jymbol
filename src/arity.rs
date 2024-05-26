use std::ops::{RangeFrom, RangeFull};

use anyhow::anyhow;

use crate::Result;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Arity {
    Exactly(usize),
    AtLeast(usize),
}

impl From<usize> for Arity {
    fn from(n: usize) -> Self {
        Arity::Exactly(n)
    }
}

impl From<RangeFrom<usize>> for Arity {
    fn from(range: RangeFrom<usize>) -> Self {
        Arity::AtLeast(range.start)
    }
}

impl From<RangeFull> for Arity {
    fn from(_: RangeFull) -> Self {
        Arity::AtLeast(0)
    }
}

impl Arity {
    pub fn check(&self, actual: usize) -> Result<()> {
        match *self {
            Arity::Exactly(expected) => {
                if actual == expected {
                    Ok(())
                } else if expected == 1 {
                    Err(anyhow!("expected {expected} argument, got {actual}"))
                } else {
                    Err(anyhow!("expected {expected} arguments, got {actual}"))
                }
            }
            Arity::AtLeast(expected) => {
                if actual >= expected {
                    Ok(())
                } else {
                    Err(anyhow!(
                        "expected {expected} or more arguments, got {actual}"
                    ))
                }
            }
        }
    }
}
