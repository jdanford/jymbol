use crate::{FnId, Symbol};

use super::op;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Instruction {
    Nop,
    Drop,
    Number(f64),
    Compound(Symbol, u8),
    Closure(FnId, u8),
    UnOp(op::Unary),
    BinOp(op::Binary),
    Get(u16, u16),
    Set(u16, u16),
    Jump(u32),
    Branch(u32, u32),
    Frame(u8),
    Ret,
}
