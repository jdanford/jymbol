use std::num::NonZeroU32;

use crate::{
    pack::{Pack, Unpacked},
    Error, Ref, Result, Symbol,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Number(f64),
    Symbol(Symbol),
    Ref(Ref),
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Number(f)
    }
}

impl From<Symbol> for Value {
    fn from(symbol: Symbol) -> Self {
        Value::Symbol(symbol)
    }
}

impl From<Ref> for Value {
    fn from(ref_: Ref) -> Self {
        Value::Ref(ref_)
    }
}

impl TryFrom<Value> for f64 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Number(n) => Ok(n),
            _ => Err(format!("expected number, got {:?}", value)),
        }
    }
}

impl TryFrom<Value> for Symbol {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Symbol(symbol) => Ok(symbol),
            _ => Err(format!("expected symbol, got {:?}", value)),
        }
    }
}

impl TryFrom<Value> for Ref {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Ref(ref_) => Ok(ref_),
            _ => Err(format!("expected ref, got {:?}", value)),
        }
    }
}

fn try_u32_from_u64(n: u64) -> Result<u32> {
    n.try_into()
        .map_err(|_| "value is too large to fit in u32".to_string())
}

const SYMBOL_TAG: u8 = 0;
const REF_TAG: u8 = 1;

impl Pack for Value {
    type Error = String;

    fn as_unpacked(&self) -> Unpacked {
        match *self {
            Value::Number(f) => Unpacked::Float(f),
            Value::Symbol(symbol) => {
                let index = u32::from(NonZeroU32::from(symbol));
                Unpacked::Tagged(SYMBOL_TAG, index.into())
            }
            Value::Ref(ref_) => {
                let index: u32 = ref_.into();
                Unpacked::Tagged(REF_TAG, index.into())
            }
        }
    }

    fn try_from_unpacked(unpacked: Unpacked) -> Result<Value> {
        match unpacked {
            Unpacked::Float(f) => Ok(f.into()),
            Unpacked::Tagged(SYMBOL_TAG, n) => {
                let index = try_u32_from_u64(n)?;
                let symbol = Symbol::try_from(index)?;
                Ok(symbol.into())
            }
            Unpacked::Tagged(REF_TAG, n) => {
                let index = try_u32_from_u64(n)?;
                let ref_ = Ref::from(index);
                Ok(ref_.into())
            }
            Unpacked::Tagged(t, _) => Err(format!("unexpected tag: {}", t)),
        }
    }
}
