use crate::{vm::Inst, Arity, FnId};

#[derive(Clone, PartialEq, Debug)]
pub struct Compiled {
    pub fn_id: FnId,
    pub arity: Arity,
    pub code: Vec<Inst>,
}
