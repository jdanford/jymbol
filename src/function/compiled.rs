use crate::{Arity, FnId, Inst};

#[derive(Clone, PartialEq, Debug)]
pub struct Compiled {
    pub fn_id: FnId,
    pub arity: Arity,
    pub code: Vec<Inst>,
}

impl Compiled {
    pub fn new<A: Into<Arity>>(fn_id: FnId, arity: A, code: Vec<Inst>) -> Self {
        Compiled {
            fn_id,
            arity: arity.into(),
            code,
        }
    }
}
