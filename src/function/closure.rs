use std::fmt::{self, Display, Formatter};

use gc::{Finalize, Trace};

use crate::{FnId, Value};

#[derive(Clone, PartialEq, Debug, Trace, Finalize)]
pub struct Closure {
    pub fn_id: FnId,
    pub values: Vec<Value>,
}

impl Display for Closure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(#fn ...)")
    }
}
