use std::fmt::{self, Display, Formatter};

use gc::{Finalize, Gc, Trace};

use crate::{Env, Symbol, Value};

#[derive(Clone, PartialEq, PartialOrd, Debug, Trace, Finalize)]
pub struct Function {
    pub env: Gc<Env>,
    pub params: Vec<Symbol>,
    pub body: Value,
}

impl Function {
    #[must_use]
    pub fn new(env: Gc<Env>, params: Vec<Symbol>, body: Value) -> Self {
        Function { env, params, body }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(#function ...)")
    }
}
