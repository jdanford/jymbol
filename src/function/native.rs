use std::{
    cmp::Ordering,
    fmt::{self, Debug, Formatter},
};

use crate::{Arity, FnId, Result, Value};

pub trait Args {
    fn checked<const N: usize>(&self) -> Result<&[Value; N]>;
}

impl Args for [Value] {
    #[allow(clippy::missing_panics_doc)]
    fn checked<const N: usize>(&self) -> Result<&[Value; N]> {
        Arity::from(N).check(self.len())?;
        Ok(self.try_into().unwrap())
    }
}

pub type RawFn = fn(&[Value]) -> Result<Value>;

#[derive(Clone)]
pub struct Native {
    pub id: FnId,
    pub arity: Arity,
    pub function: Box<RawFn>,
}

impl Native {
    pub fn new<A: Into<Arity>>(id: FnId, function: RawFn, arity: A) -> Native {
        Native {
            id,
            arity: arity.into(),
            function: function.into(),
        }
    }

    pub fn apply(&self, args: &[Value]) -> Result<Value> {
        self.arity.check(args.len())?;
        (self.function)(args)
    }
}

impl PartialEq for Native {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl PartialOrd for Native {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None
    }
}

impl Debug for Native {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Native")
            .field("id", &self.id)
            .field("arity", &self.arity)
            .finish_non_exhaustive()
    }
}
