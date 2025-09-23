use std::fmt::{self, Display, Formatter};

use dumpster::Trace;

use crate::{FnId, Value};

#[derive(Clone, PartialEq, Debug, Trace)]
pub struct Closure {
    pub fn_id: FnId,
    pub values: Vec<Value>,
}

impl Display for Closure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(#fn ...)")
    }
}
