pub mod op;

use crate::{FnId, Symbol, Value};

#[derive(Clone, PartialEq, Debug)]
pub enum Inst {
    Nop,
    Drop,
    Value(Value),
    List(u16),
    Compound(Symbol, u16),
    Closure(FnId, u16),
    UnOp(op::Unary),
    BinOp(op::Binary),
    Get(u16, u16),
    Set(u16, u16),
    Jump(u32),
    JumpIf(u32),
    JumpIfNot(u32),
    Call(u16),
    Return,
    Recur(u16, u32),
}
