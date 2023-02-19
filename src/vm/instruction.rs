use crate::{FnId, Symbol, Value};

use super::op;

#[derive(Clone, PartialEq, Debug)]
pub enum Inst {
    Nop,
    Drop,
    Value(Value),
    Compound(Symbol, u8),
    Closure(FnId, u8),
    UnOp(op::Unary),
    BinOp(op::Binary),
    Get(u16, u16),
    Set(u16, u16),
    Jump(u32),
    JumpIf(u32),
    JumpIfNot(u32),
    Call(u8),
    Ret,
}
